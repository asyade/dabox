//! A memory-backed implementation of the `DaRepository` trait.
//! The implementation is thread-safe and use a semi lock-free approach to archive good async performance.

use crate::prelude::*;

/// Internal representation of a `DaDirectory` in the memory backend.
#[derive(Debug, Clone)]
struct MemDaDirectory {
    /// Unique identifier for the directory
    sid: DaDirectorySid,
    /// Name of the directory
    name: Arc<RwLock<String>>,
    /// Parent directory unique identifier (none if the directory is the root)
    parent_sid: Option<DaDirectorySid>,
    /// Children directory identifiers
    children: Arc<RwLock<HashSet<DaDirectorySid>>>,
    /// The depth of the directory in the hierarchy
    depth: u32,
}

/// A memory-backed implementation of the `DaRepositoryExt` trait.
/// Useful for testing and development purposes.
#[derive(Clone)]
pub struct MemRepository {
    buckets: Arc<RwLock<HashMap<EntityUid, Bucket>>>,
}

#[derive(Clone)]
struct Bucket {
    sid_counter: Arc<AtomicI64>,
    directories: Arc<RwLock<BTreeMap<DaDirectorySid, MemDaDirectory>>>,
}

impl MemRepository {
    pub fn new() -> Self {
        Self {
            buckets: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    async fn bucket(&self, uid: EntityUid) -> Bucket {
        let maybe_bucket = {
            let lock = self.buckets.read().await;
            lock.get(&uid).cloned()
        };
        match maybe_bucket {
            Some(bucket) => bucket.clone(),
            None => {
                let bucket = Bucket::new();
                let mut lock = self.buckets.write().await;
                lock.insert(uid, bucket);
                lock.get(&uid).unwrap().clone()
            }
        }
    }
}

impl DaRepository for MemRepository {
    async fn create_directory(
        &self,
        requested_by: EntityUid,
        name: &str,
        parent: Option<DaDirectorySid>,
    ) -> DaResult<DaDirectory> {
        let bucket = self.bucket(requested_by).await;

        // Generate a new sid from the global directory counter
        let sid = DaDirectorySid(
            bucket
                .sid_counter
                .fetch_add(1, std::sync::atomic::Ordering::SeqCst),
        );

        // Create in-memory representation of the directory
        let mut directory = MemDaDirectory {
            sid: sid,
            name: Arc::new(RwLock::new(name.to_string())),
            parent_sid: parent,
            children: Arc::new(RwLock::new(HashSet::new())),
            depth: 0,
        };

        // Add the directory to both the global map and the parent's children list
        // *thread safety* we use a block here to limit the scope of the global map lock
        {
            let mut directories_lock = bucket.directories.write().await;
            if let Some(parent_sid) = parent {
                let parent_dir = directories_lock
                    .get(&parent_sid)
                    .ok_or(DaError::DirectoryNotFound(parent_sid))?;
                parent_dir.children.write().await.insert(sid);
                directory.depth = parent_dir.depth + 1;
            }

            if directories_lock.insert(sid, directory.clone()).is_some() {
                panic!("Directory sid collision (sid sequence is broken)");
            }
        }

        // Get the directory from the in-memory storage
        let created_dir = self.get_directory(requested_by, sid).await;

        // Ensure the directory is immediately available after creation
        assert!(
            created_dir.is_ok(),
            "Directory should be immediately available after creation"
        );
        created_dir
    }

    async fn delete_directory(&self, requested_by: EntityUid, id: DaDirectorySid) -> DaResult<()> {
        let bucket = self.bucket(requested_by).await;

        // To ensure that there is no race-condition such as a child created between the get of all children and the removal of the directory itself
        // we need to lock the global map exclusively. Better approach would be to use a scoped approach but we will keep it simple for now.
        let mut dirs_lock = bucket.directories.write().await;

        let parent_sid = dirs_lock
            .get(&id)
            .ok_or(DaError::DirectoryNotFound(id))?
            .parent_sid;

        let mut to_be_removed = vec![id];

        while let Some(sid) = to_be_removed.pop() {
            if let Some(dir) = dirs_lock.remove(&sid) {
                to_be_removed.extend(dir.children.read().await.iter());
            } else {
                return Err(DaError::DirectoryNotFound(sid));
            }
        }

        for sid in to_be_removed {
            dirs_lock.remove(&sid);
        }

        // Remove the directory from the parent's children list
        // *note* we're doing this after the loop to ensure that we are able to remove orphaned children
        if let Some(parent_sid) = parent_sid {
            dirs_lock
                .get_mut(&parent_sid)
                .ok_or(DaError::DirectoryNotFound(parent_sid))?
                .children
                .write()
                .await
                .remove(&id);
        }
        Ok(())
    }

    async fn rename_directory(
        &self,
        requested_by: EntityUid,
        id: DaDirectorySid,
        new_name: &str,
    ) -> DaResult<()> {
        let bucket = self.bucket(requested_by).await;
        let dirs_lock = bucket.directories.read().await;
        let dir_lock = dirs_lock.get(&id).ok_or(DaError::DirectoryNotFound(id))?;
        let mut name_lock = dir_lock.name.write().await;
        *name_lock = new_name.to_string();
        Ok(())
    }

    async fn get_directory(
        &self,
        requested_by: EntityUid,
        id: DaDirectorySid,
    ) -> DaResult<DaDirectory> {
        self.clone().read_dir(requested_by, id).await
    }
}

impl MemRepository {
    #[async_recursion::async_recursion]
    async fn read_dir(self, requested_by: EntityUid, id: DaDirectorySid) -> DaResult<DaDirectory> {
        let bucket = self.bucket(requested_by).await;

        let dir = {
            let dirs_lock = bucket.directories.read().await;
            dirs_lock
                .get(&id)
                .ok_or(DaError::DirectoryNotFound(id))?
                .clone()
        };
        let name = dir.name.read().await.clone();

        // Get a copy of all the children sids
        let children_sids = dir.children.read().await.clone();

        // Create a stream of futures that will each resolve to a `DaDirectory`
        let children = futures::stream::iter(
            children_sids
                .into_iter()
                .map(|sid| self.get_directory(requested_by, sid)),
        )
        // Buffer up to 16 futures at a time to avoid excessive memory allocations (even if in our actual use case children aggreation is simple we might want to add more complex logic in the future that inevitability will require buffering)
        .buffer_unordered(16)
        // Filter out any errors and return the directories
        .filter_map(|res| async move {
            match res {
                Ok(dir) => Some(dir),
                Err(e) => {
                    eprintln!("Error getting directory: {}", e);
                    None
                }
            }
        });

        Ok(DaDirectory {
            name,
            depth: dir.depth,
            sid: dir.sid,
            parent_sid: dir.parent_sid,
            children: children.collect().await,
        })
    }
}

impl Bucket {
    pub fn new() -> Self {
        Self {
            sid_counter: Arc::new(AtomicI64::new(0)),
            directories: Arc::new(RwLock::new(BTreeMap::new())),
        }
    }
}

/// Tests for the memory backend
/// # Todo
/// - Add tests for concurrency
/// - Add tests to ensure that directory are well isolated by user
#[cfg(test)]
mod tests {
    use crate::{entity::StaticEntity, prelude::*};

    #[tokio::test]
    async fn test_directory_creation_basic() {
        let repo = MemRepository::new();
        let mut known_sids = HashMap::new();
        for idx in 0..1024 {
            let name = format!("test-{}", idx);
            let dir = repo
                .create_directory(StaticEntity::root().uid(), &name, None)
                .await
                .unwrap();
            assert!(!known_sids.contains_key(&dir.sid), "sid sequence broken");
            known_sids.insert(dir.sid, name);
        }
        for (id, name) in known_sids.clone() {
            let dir = repo
                .get_directory(StaticEntity::root().uid(), id)
                .await
                .expect("unable to get created directory");
            assert_eq!(dir.name, name);
        }
        for (id, _) in known_sids {
            repo.delete_directory(StaticEntity::root().uid(), id)
                .await
                .unwrap();
        }
    }

    #[tokio::test]
    async fn test_directory_hierarchy() {
        async fn tree(repo: &MemRepository, depth: u32) -> (DaDirectory, Vec<DaDirectory>) {
            let root = repo
                .create_directory(StaticEntity::root().uid(), "root", None)
                .await
                .unwrap();
            let mut dir = root.clone();
            let mut children = vec![];
            for depth in 0..depth {
                let child = repo
                    .create_directory(
                        StaticEntity::root().uid(),
                        &format!("child-{}", depth),
                        Some(dir.sid),
                    )
                    .await
                    .unwrap();
                children.push(child.clone());
                dir = child;
            }
            (
                repo.get_directory(StaticEntity::root().uid(), root.sid)
                    .await
                    .unwrap(),
                children,
            )
        }

        let repo = MemRepository::new();
        let (root, children) = tree(&repo, 16).await;
        assert_eq!(children.last().unwrap().depth, 16);

        repo.delete_directory(StaticEntity::root().uid(), children[10].sid)
            .await
            .unwrap();
        for child in children.iter().skip(10) {
            assert!(repo
                .get_directory(StaticEntity::root().uid(), child.sid)
                .await
                .is_err());
        }
        repo.delete_directory(StaticEntity::root().uid(), children[9].sid)
            .await
            .unwrap();
        assert!(repo
            .get_directory(StaticEntity::root().uid(), children[9].sid)
            .await
            .is_err());
        repo.delete_directory(StaticEntity::root().uid(), root.sid)
            .await
            .unwrap();
        assert!(repo
            .get_directory(StaticEntity::root().uid(), root.sid)
            .await
            .is_err());
    }
}
