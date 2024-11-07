use crate::prelude::*;

const DIRECTORY_MAX_DEPTH: u32 = 4096;

/// Internal representation of a `DaDirectory` in the memory backend.
/// Thread-safe.
#[derive(Debug, Clone)]
struct MemDaDirectory {
    /// Unique identifier for the directory
    sid: DaDirectorySid,
    /// Name of the directory
    name: Arc<RwLock<String>>,
    /// Parent directory unique identifier (none if the directory is the root)
    parent_sid: Option<DaDirectorySid>,
    /// Children directory identifiers
    children: Arc<RwLock<Vec<DaDirectorySid>>>,
    /// The depth of the directory in the hierarchy
    depth: u32,
}

/// A memory-backed implementation of the `DaRepositoryExt` trait.
/// Useful for testing and development purposes.
#[derive(Clone)]
pub struct MemRepository {
    /// Global counter for generating new directory serial identifiers
    sid_counter: Arc<AtomicI64>,
    /// In-memory storage of all directories
    directories: Arc<RwLock<HashMap<DaDirectorySid, MemDaDirectory>>>,
}

impl MemRepository {
    pub fn new() -> Self {
        Self {
            sid_counter: Arc::new(AtomicI64::new(0)),
            directories: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl DaRepositoryExt for MemRepository {
    async fn create_directory(
        &self,
        name: &str,
        parent: Option<DaDirectorySid>,
    ) -> DaResult<DaDirectory> {
        // Generate a new sid from the global directory counter
        let sid = DaDirectorySid(
            self.sid_counter
                .fetch_add(1, std::sync::atomic::Ordering::SeqCst),
        );

        // Create in-memory representation of the directory
        let mut directory = MemDaDirectory {
            sid: sid,
            name: Arc::new(RwLock::new(name.to_string())),
            parent_sid: parent,
            children: Arc::new(RwLock::new(vec![])),
            depth: 0,
        };

        // Add the directory to both the global map and the parent's children list
        // *thread safety* we use a block here to limit the scope of the global map lock
        {
            let mut directories_lock = self.directories.write().await;
            if let Some(parent_sid) = parent {
                let parent_dir = directories_lock
                    .get(&parent_sid)
                    .ok_or(DaError::DirectoryNotFound(parent_sid))?;
                parent_dir.children.write().await.push(sid);
                directory.depth = parent_dir.depth + 1;
            }

            if directories_lock.insert(sid, directory.clone()).is_some() {
                panic!("Directory sid collision (sid sequence is broken)");
            }
        }

        // Get the directory from the in-memory storage
        let created_dir = self.get_directory(sid).await;

        // Ensure the directory is immediately available after creation
        assert!(
            created_dir.is_ok(),
            "Directory should be immediately available after creation"
        );
        created_dir
    }

    async fn delete_directory(&self, id: DaDirectorySid) -> DaResult<()> {
        if let Some(parent) = self.get_directory(id).await?.parent_sid {}
        if let None = self.directories.write().await.remove(&id) {
            return Err(DaError::DirectoryNotFound(id));
        }
        Ok(())
    }

    async fn rename_directory(&self, id: DaDirectorySid, new_name: &str) -> DaResult<()> {
        let dirs_lock = self.directories.read().await;
        let dir_lock = dirs_lock.get(&id).ok_or(DaError::DirectoryNotFound(id))?;
        let mut name_lock = dir_lock.name.write().await;
        *name_lock = new_name.to_string();
        Ok(())
    }

    async fn get_directory(&self, id: DaDirectorySid) -> DaResult<DaDirectory> {
        self.clone().read_dir(id).await
    }
}

impl MemRepository {
    #[async_recursion::async_recursion]
    async fn read_dir(self, id: DaDirectorySid) -> DaResult<DaDirectory> {
        let dir = {
            let dirs_lock = self.directories.read().await;
            dirs_lock
                .get(&id)
                .ok_or(DaError::DirectoryNotFound(id))?
                .clone()
        };
        let name = dir.name.read().await.clone();

        Ok(DaDirectory {
            name,
            depth: dir.depth,
            sid: dir.sid,
            parent_sid: dir.parent_sid,
            children: dir.view_children(&self).await.collect().await,
        })
    }
}

impl MemDaDirectory {
    /// Returns a stream of the directory's children.
    /// This function may seems a bit overkill, but it's a good starting point for
    /// that allows great extensibility with asynchronous operations capabilities.
    async fn view_children<'a>(
        &'a self,
        repository: &'a MemRepository,
    ) -> impl Stream<Item = DaDirectory> + 'a {
        // Get a copy of all the children sids
        let children_sids = self.children.read().await.clone();

        // Create a stream of futures that will each resolve to a `DaDirectory`
        futures::stream::iter(
            children_sids
                .into_iter()
                .map(|sid| repository.get_directory(sid)),
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
        })
    }
}
