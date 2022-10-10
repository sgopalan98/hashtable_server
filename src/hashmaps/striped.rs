use std::sync::{RwLock, Mutex};

pub struct StripedHashTable {
    table: RwLock<Vec<Mutex<Vec<(usize, usize, usize)>>>>
}

impl StripedHashTable {
    pub fn with_capacity(capacity: usize) -> StripedHashTable {
        let mut buckets = vec![];
        for _ in 0..capacity {
            buckets.push(Mutex::new(Vec::new()));
        }
        StripedHashTable { table: RwLock::new(buckets) }
    }

    pub fn get(&self, key: usize) -> Result<usize, usize>{
        let buckets = self.table.read().unwrap();
        let index = key as usize % buckets.len();
        let bucket = buckets[index].lock().unwrap();
        let mut found = false;
        let mut value = 0;
        for i in 0..bucket.len() {
            if bucket[i].2 == 1 {
                let a_key = bucket[i].0;
                if key == a_key {
                    found = true;
                    value = bucket[i].1;
                    break;
                }
            }
        }
        if found == false {
            return Err(value);
        }
        else {
            return Ok(value);
        }
    }

    pub fn insert_or_update(&self, key: usize, value: usize) -> Result<i32, i32> {
        let buckets = self.table.read().unwrap();
        let index = key as usize % buckets.len();
        let mut bucket = buckets[index].lock().unwrap();
        if bucket.len() >= buckets.len() {
            drop(bucket);
            drop(buckets);
            self.resize();
            return self.insert_or_update(key, value);
        }
        let mut found = false;
        for i in 0..bucket.len() {
            if bucket[i].2 == 1 {
                let a_key = bucket[i].0;
                if key == a_key {
                    found = true;
                    bucket[i].1 = value;
                    break;
                }
            }
        }
        if !found {
            bucket.push((key, value, 1));
        }
        return Ok(0);
    }


    fn resize(&self) {
        let mut old_buckets = self.table.write().unwrap();
        let a_bucket = old_buckets[0].lock().unwrap();
        
        if a_bucket.len() >= old_buckets.len(){
            drop(a_bucket);
            let mut buckets:Vec<Mutex<Vec<(usize, usize, usize)>>> = Vec::new();
            let old_capacity = old_buckets.len();
            let new_capacity = old_buckets.len() * 2;
            for _ in 0..new_capacity {
                buckets.push(Mutex::new(Vec::new()));
            }
            for i in 0..old_capacity {
                let old_bucket = old_buckets[i].lock().unwrap();
                for i in 0..old_bucket.len() {
                    let new_index = old_bucket[i].0 as usize % new_capacity;
                    let mut new_bucket = buckets[new_index as usize].lock().unwrap();
                    new_bucket.push((old_bucket[i].0, old_bucket[i].1, old_bucket[i].2));
                }
            }
            *old_buckets = buckets;
        }
        else {
            
        }
        return;
    }


    pub fn remove(&self, key: usize) -> Result<usize, usize> {
        let buckets = self.table.read().unwrap();
        let index = key as usize % buckets.len();
        let mut bucket = buckets[index].lock().unwrap();
        let mut found = false;
        for i in 0..bucket.len() {
            if bucket[i].2 == 1 {
                let a_key = bucket[i].0;
                if key == a_key {
                    found = true;
                    bucket[i].2 = 0;
                    break;
                }
            }
        }
        if found {
            return Ok(0);
        }
        return Err(0);
    }

}