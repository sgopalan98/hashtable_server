use std::sync::Arc;

use bustle::{Collection, CollectionHandle};
use crate::hashmaps::striped::{StripedHashTable};

#[derive(Clone)]
pub struct StripedHashMapTable(Arc<StripedHashTable>);

impl Collection for StripedHashMapTable {
    type Handle = Self;

    fn with_capacity(capacity: usize) -> Self {
        Self(Arc::new(StripedHashTable::with_capacity(capacity)))
    }
    fn pin(&self) -> Self::Handle {
        return self.clone();
    }
}


impl CollectionHandle for StripedHashMapTable {

    type Key = u64;

    fn get(&mut self, key: &Self::Key) -> bool {
        self.0.get(*key as usize).is_ok()
    }

    fn insert(&mut self, key: &Self::Key) -> bool {
        self.0.insert_or_update(*key as usize, 0).is_ok()
    }

    fn remove(&mut self, key: &Self::Key) -> bool {
        match self.0.remove(*key as usize) {
            Ok(_) => return true,
            Err(_) => return false
        }
    }

    fn update(&mut self, key: &Self::Key) -> bool {
        match self.0.get(*key as usize){
            Ok(value) => {
                return self.0.insert_or_update(*key as usize, value + 1).is_ok();
            }
            Err(_value) => {
                return false;
            }
        }
    }

}
