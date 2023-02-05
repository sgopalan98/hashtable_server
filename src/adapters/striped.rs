use std::{error, sync::{Arc, Mutex}, collections::{HashMap, HashSet}};

use crate::Adapter;

pub struct StripedHashMapAdapter(Arc<Vec<Mutex<HashMap<u64, u64>>>>);

impl Adapter for StripedHashMapAdapter {
    type Key = u64;
    type Value = u64;

    fn create_with_capacity(no_buckets: usize) -> Self {
        let mut buckets = Vec::new();
        for i in 0..no_buckets {
            buckets.push(Mutex::new(HashMap::new()));
        }
        Self(Arc::new(buckets))
    }

    fn clone(&self) -> Self {
        let map = &self.0;
        Self(Arc::clone(&map))
    }

    fn get(&mut self, key: &Self::Key) -> bool {
        let buckets = &self.0;
        let index = *key as usize % buckets.len();
        let bucket = buckets[index].lock().unwrap();
        bucket.get(key).is_some()
    }

    fn insert(&mut self, key: &Self::Key, value: Self::Value) -> bool {
        let buckets = &self.0;
        let index = *key as usize % buckets.len();
        let mut bucket = buckets[index].lock().unwrap();
        bucket.insert(*key, value).is_none()
    }

    fn remove(&mut self, key: &Self::Key) -> bool {
        let buckets = &self.0;
        let index = *key as usize % buckets.len();
        let mut bucket = buckets[index].lock().unwrap();
        bucket.remove(key).is_some()
    }

    fn update(&mut self, key: &Self::Key) -> bool {
        let buckets = &self.0;
        let index = *key as usize % buckets.len();
        let mut bucket = buckets[index].lock().unwrap();
        bucket.get_mut(key).map(|mut v| *v += 1).is_some()
    }
}
