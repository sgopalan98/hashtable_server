use std::{error, sync::{Arc, Mutex, RwLock}, collections::{HashMap, HashSet}};
use once_cell::sync::OnceCell;

use crate::Adapter;

pub struct StripedHashMapAdapter(Arc<Vec<RwLock<HashMap<u64, u64>>>>);

impl Adapter for StripedHashMapAdapter {
    type Key = u64;
    type Value = u64;

    fn create_with_capacity(capacity: usize) -> Self {
        let no_buckets = (num_cpus::get() * 4).next_power_of_two();
        println!("No of buckets = {}", no_buckets);
        let capacity_per_bucket = capacity / no_buckets;
        let mut buckets = Vec::new();
        for _i in 0..no_buckets {
            buckets.push(RwLock::new(HashMap::with_capacity(capacity_per_bucket)));
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
        let bucket = buckets[index].read().unwrap();
        bucket.get(key).is_some()
    }

    fn insert(&mut self, key: &Self::Key, value: Self::Value) -> bool {
        let buckets = &self.0;
        let index = *key as usize % buckets.len();
        let mut bucket = buckets[index].write().unwrap();
        bucket.insert(*key, value).is_none()
    }

    fn remove(&mut self, key: &Self::Key) -> bool {
        let buckets = &self.0;
        let index = *key as usize % buckets.len();
        let mut bucket = buckets[index].write().unwrap();
        bucket.remove(key).is_some()
    }

    fn update(&mut self, key: &Self::Key) -> bool {
        let buckets = &self.0;
        let index = *key as usize % buckets.len();
        let mut bucket = buckets[index].write().unwrap();
        bucket.get_mut(key).map(|mut v| *v += 1).is_some()
    }
}
