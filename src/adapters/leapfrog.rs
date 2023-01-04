use std::{sync::Arc, error};

use leapfrog::LeapMap;
use crate::Adapter;

pub struct LeapMapAdapter(Arc<LeapMap<u64, u64>>);

impl Adapter for LeapMapAdapter {
    type Key = u64;
    type Value = u64;

    fn create_with_capacity(capacity: usize) -> Self {
        Self(Arc::new(LeapMap::with_capacity(capacity)))
    }

    fn clone(&self) -> Self {
        let map = &self.0;
        Self(Arc::clone(&map))
    }

    fn get(&mut self, key: &Self::Key) -> bool {
        self.0.get(key).is_some()
    }

    fn insert(&mut self, key: &Self::Key, value: Self::Value) -> bool {
        self.0.insert(*key, value).is_none()
    }

    fn remove(&mut self, key: &Self::Key) -> bool {
        self.0.remove(key).is_some()
    }

    fn update(&mut self, key: &Self::Key) -> bool {
        match self.0.get_mut(key) {
            Some(mut val_ref) => return val_ref.update(|val: &mut Self::Value| *val += 1).is_some(),
            None => {
                return false;
            }
        };
    }
}