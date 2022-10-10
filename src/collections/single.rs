use std::collections::HashMap;
use std::hash::{BuildHasher, Hash};
use std::sync::{Arc, RwLock};

use bustle::*;


#[derive(Clone)]
pub struct RwLockStdHashMapTable(Arc<RwLock<HashMap<u64, u64>>>);

impl Collection for RwLockStdHashMapTable
{
    type Handle = Self;

    fn with_capacity(capacity: usize) -> Self {
        Self(Arc::new(RwLock::new(HashMap::with_capacity(
            capacity
        ))))
    }

    fn pin(&self) -> Self::Handle {
        self.clone()
    }
}

impl CollectionHandle for RwLockStdHashMapTable
{
    type Key = u64;

    fn get(&mut self, key: &Self::Key) -> bool {
        self.0.read().unwrap().get(key).is_some()
    }

    fn insert(&mut self, key: &Self::Key) -> bool {
        self.0.write().unwrap().insert(*key, 0).is_none()
    }

    fn remove(&mut self, key: &Self::Key) -> bool {
        self.0.write().unwrap().remove(key).is_some()
    }

    fn update(&mut self, key: &Self::Key) -> bool {
        let mut map = self.0.write().unwrap();
        map.get_mut(key).map(|v| *v += 1).is_some()
    }
}