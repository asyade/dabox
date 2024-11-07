use std::collections::{HashMap, HashSet};

use dabox_core::prelude::*;

#[tokio::test]
async fn test_directory_creation_basic() {
    let repo = MemRepository::new();
    let mut known_sids = HashMap::new();
    for idx in 0..1024 {
        let name = format!("test-{}", idx);
        let dir = repo.create_directory(&name, None).await.unwrap();
        assert!(!known_sids.contains_key(&dir.sid), "sid sequence broken");
        known_sids.insert(dir.sid, name);
    }
    for (id, name) in known_sids.into_iter() {
        let dir = repo
            .get_directory(id)
            .await
            .expect("unable to get created directory");
        assert_eq!(dir.name, name);
    }
}

#[tokio::test]
async fn test_directory_hierarchy() {
    async fn tree(repo: &MemRepository, depth: u32) -> (DaDirectory, Vec<DaDirectory>) {
        let root = repo.create_directory("root", None).await.unwrap();
        let mut dir = root.clone();
        let mut children = vec![];
        for depth in 0..depth {
            let child = repo
                .create_directory(&format!("child-{}", depth), Some(dir.sid))
                .await
                .unwrap();
            children.push(child.clone());
            dir = child;
        }
        (repo.get_directory(root.sid).await.unwrap(), children)
    }

    let repo = MemRepository::new();
    let (root, children) = tree(&repo, 128).await;
    assert_eq!(children.last().unwrap().depth, 128);
}
