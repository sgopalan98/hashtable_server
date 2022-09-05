use std::{sync::Mutex};


pub struct HashTable{
    locks: Vec<Mutex<Vec<(i32, i32)>>>,
    capacity: i32
}

impl HashTable{
    pub fn new(capacity: i32) -> HashTable {
        let mut hash_list = Vec::new();
        for i in 0..capacity {
            let new_list = Mutex::new(Vec::new());
            hash_list.push(new_list);
        }
        HashTable {
            locks: hash_list,
            capacity
        }
    }

    pub fn get(&self, key: i32) -> Result<i32, i32> {
        let bucket_index = key % self.capacity;
        let lock = &self.locks[bucket_index as usize];
        let bucket = lock.lock().unwrap();
        let mut found = false;
        let mut value = -1;
        for i in 0..bucket.len() {
            let a_key = bucket[i].0;
            if key == a_key {
                found = true;
                value = bucket[i].1;
                break;
            }
        }
        if found == false {
            return Err(value);
        }
        else {
            return Ok(value);
        }
    }

    pub fn contains(self, key: i32) -> bool {
        let bucket_index = key % self.capacity;
        let lock = &self.locks[bucket_index as usize];
        let bucket = lock.lock().unwrap();
        let mut found = false;
        for i in 0..bucket.len() {
            let a_key = bucket[i].0;
            if key == a_key {
                found = true;
                break;
            }
        }
        return found;
    }

    pub fn put(&self, key: i32, value: i32) -> Result<i32, i32> {
        let bucket_index = key % self.capacity;
        let lock = &self.locks[bucket_index as usize];
        let mut bucket = lock.lock().unwrap();
        let mut found = false;
        for i in 0..bucket.len() {
            let a_key = bucket[i].0;
            if key == a_key {
                found = true;
                bucket[i].1 = value;
            }
        }
        if !found {
            bucket.push((key, value));
        }
        return Ok(0);
    }
}