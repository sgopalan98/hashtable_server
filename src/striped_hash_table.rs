use std::{sync::Mutex};


pub struct HashTable{
    locks: Vec<Mutex<i32>>,
    data: Vec<Mutex<Vec<(i32, i32)>>>,
    capacity: i32
}

impl HashTable{
    pub fn new(capacity: i32) -> HashTable {
        let mut locks = Vec::new();
        let mut data = Vec::with_capacity(capacity as usize);
        for i in 0..capacity {
            let new_lock = Mutex::new(0);
            locks.push(new_lock);
        }
        HashTable {
            locks,
            data,
            capacity
        }
    }

    pub fn get(&self, key: i32) -> Result<i32, i32> {
        let lock_index = (key as usize) % self.locks.len();
        let lock = &self.locks[lock_index];
        lock.lock().unwrap();
        let bucket_index = key % self.capacity;
        let bucket = &self.data[bucket_index as usize].lock().unwrap();
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
        let lock_index = (key as usize) % self.locks.len();
        let lock = &self.locks[lock_index];
        lock.lock().unwrap();
        let bucket_index = key % self.capacity;
        let bucket = &self.data[bucket_index as usize].lock().unwrap();
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

    pub fn put(&mut self, key: i32, value: i32) -> Result<i32, i32> {
        let lock_index = (key as usize) % self.locks.len();
        let lock = &self.locks[lock_index];
        lock.lock().unwrap();
        let bucket_index = key % self.capacity;
        let bucket = &self.data[bucket_index as usize].lock().unwrap();
        if bucket.len() > (self.capacity as usize) * 4 {
            self.capacity = self.capacity * 2;
        }
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