use std::sync::RwLock;

pub struct StripedHashTable {
    buckets: Vec<RwLock<Vec<(usize, usize, usize)>>>
}

impl StripedHashTable {
    pub fn with_capacity(capacity: usize) -> StripedHashTable {
        let mut buckets = vec![];
        for _ in 0..capacity {
            buckets.push(RwLock::new(Vec::new()));
        }
        StripedHashTable { buckets }
    }

    pub fn get(&self, key: usize) -> Result<usize, usize>{
        let index = key as usize % self.buckets.len();
        let bucket = self.buckets[index].read().unwrap();
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
        let index = key as usize % self.buckets.len();
        let mut bucket = self.buckets[index].write().unwrap();
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



    pub fn remove(&self, key: usize) -> Result<usize, usize> {
        let index = key as usize % self.buckets.len();
        let mut bucket = self.buckets[index].write().unwrap();
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