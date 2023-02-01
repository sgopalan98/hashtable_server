use std::{error, sync::{Arc, Mutex, MutexGuard}, collections::HashMap};

use crate::Adapter;

pub struct SingleLockMapAdapter(Arc<Mutex<HashMap<u64, u64>>>);

impl Adapter for SingleLockMapAdapter {
    type Key = u64;
    type Value = u64;

    fn create_with_capacity(capacity: usize) -> Self {
        Self(Arc::new(Mutex::new(HashMap::with_capacity(capacity))))
    }

    fn clone(&self) -> Self {
        let map = &self.0;
        Self(Arc::clone(&map))
    }

    fn get(&mut self, key: &Self::Key) -> bool {
        self.0.lock().unwrap().get(key).is_some()
    }

    fn insert(&mut self, key: &Self::Key, value: Self::Value) -> bool {
        self.0.lock().unwrap().insert(*key, value).is_none()
    }

    fn remove(&mut self, key: &Self::Key) -> bool {
        self.0.lock().unwrap().remove(key).is_some()
    }

    fn update(&mut self, key: &Self::Key) -> bool {
        self.0.lock().unwrap().get_mut(key).map(|mut v| *v += 1).is_some()
    }
    
}
