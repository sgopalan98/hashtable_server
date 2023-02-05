use std::{error, sync::{Arc, Mutex}, collections::{HashMap, HashSet}};
use once_cell::sync::OnceCell;

use crate::Adapter;

pub struct StripedHashMapAdapter(Arc<Vec<Mutex<HashMap<u64, u64>>>>);

impl Adapter for StripedHashMapAdapter {
    type Key = u64;
    type Value = u64;

    fn create_with_capacity(capacity: usize) -> Self {
        static DEFAULT_SHARD_AMOUNT: OnceCell<usize> = OnceCell::new();
        let no_buckets = *DEFAULT_SHARD_AMOUNT.get_or_init(|| {
            (std::thread::available_parallelism().map_or(1, usize::from) * 4).next_power_of_two()
        });
        let mut buckets = Vec::new();
        for _i in 0..no_buckets {
            buckets.push(Mutex::new(HashMap::with_capacity(capacity)));
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
