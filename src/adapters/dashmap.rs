use std::{error, sync::Arc};

use crate::Adapter;
use dashmap::DashMap;

pub struct DashMapAdapter(Arc<DashMap<u64, u64>>);

impl Adapter for DashMapAdapter {
    type Key = u64;
    type Value = u64;

    #[inline(never)]
    fn create_with_capacity(capacity: usize) -> Self {
        Self(Arc::new(DashMap::with_capacity(capacity)))
    }

    #[inline(never)]
    fn clone(&self) -> Self {
        let map = &self.0;
        Self(Arc::clone(&map))
    }

    #[inline(never)]
    fn get(&mut self, key: &Self::Key) -> bool {
        self.0.get(key).is_some()
    }

    #[inline(never)]
    fn insert(&mut self, key: &Self::Key, value: Self::Value) -> bool {
        self.0.insert(*key, value).is_none()
    }

    #[inline(never)]
    fn remove(&mut self, key: &Self::Key) -> bool {
        self.0.remove(key).is_some()
    }

    #[inline(never)]
    fn update(&mut self, key: &Self::Key) -> bool {
        self.0.get_mut(key).map(|mut v| *v += 1).is_some()
    }
}
