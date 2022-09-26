use std::io::Read;
use std::sync::{Arc, Mutex, RwLock};
use std::net::TcpStream;
use std::thread;
use std::io::Write;
use crate::tcp_helper;

fn convert_string_to_int(string: String) -> i32{
    return string.parse::<i32>().unwrap();
}

fn get(thread_locked_table: &Arc<RwLock<Vec<Mutex<Vec<(i32, i32)>>>>>, key: i32) -> Result<i32, i32> {
    let buckets = thread_locked_table.read().unwrap();
    let index = key as usize % buckets.len();
    let bucket = buckets[index].lock().unwrap();
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

fn resize(thread_locked_table: &Arc<RwLock<Vec<Mutex<Vec<(i32, i32)>>>>>) {
    let mut old_buckets = thread_locked_table.write().unwrap();
    let a_bucket = old_buckets[0].lock().unwrap();
    
    if a_bucket.len() >= old_buckets.len(){
        drop(a_bucket);
        let mut buckets:Vec<Mutex<Vec<(i32, i32)>>> = Vec::new();
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
                new_bucket.push((old_bucket[i].0, old_bucket[i].1));
            }
        }
        *old_buckets = buckets;
    }
    else {
        
    }
    return;
}

fn put(thread_locked_table: &Arc<RwLock<Vec<Mutex<Vec<(i32, i32)>>>>>, key: i32, value: i32) -> Result<i32, i32> {
    let buckets = thread_locked_table.read().unwrap();
    let index = key as usize % buckets.len();
    let mut bucket = buckets[index].lock().unwrap();
    if bucket.len() >= buckets.len() {
        drop(bucket);
        drop(buckets);
        resize(&thread_locked_table);
        return put(thread_locked_table, key, value);
    }
    let mut found = false;
    for i in 0..bucket.len() {
        let a_key = bucket[i].0;
        if key == a_key {
            found = true;
            bucket[i].1 = value;
            break;
        }
    }
    if !found {
        bucket.push((key, value));
    }
    return Ok(0);
}

fn reset(thread_locked_table: &Arc<RwLock<Vec<Mutex<Vec<(i32, i32)>>>>>, capacity: i32) {
    let mut old_buckets = thread_locked_table.write().unwrap();
    
    let mut buckets = vec![];
    for _ in 0..capacity {
        buckets.push(Mutex::new(Vec::new()));
    }
    
    *old_buckets = buckets;
}


fn write_string(stream: &mut TcpStream, output: String) {
    stream.write(output.as_bytes()).unwrap();
}



pub fn handle(mut stream: TcpStream, thread_locked_table: Arc<RwLock<Vec<Mutex<Vec<(i32, i32)>>>>>, capacity: i32) {
    read_command(stream, thread_locked_table, capacity)
}


fn read_command(stream: TcpStream, thread_locked_table: Arc<RwLock<Vec<Mutex<Vec<(i32, i32)>>>>>, capacity: i32){
    // Setting as non blocking
    stream.set_nonblocking(true).expect("Setting non blocking failed ");
    
    let mut stream_cloned = stream.try_clone().expect("STREAM CLONE FAILED");
    loop {
        let mut input = String::new();
        let result = stream_cloned.read_to_string(&mut input);
        match result {
            Ok(_) => {
                if input.trim().eq("CLOSE") {
                    return;
                }
                let table = Arc::clone(&thread_locked_table);
                thread::spawn(move || process(&mut stream_cloned, table, capacity, input));
            },
            Err(_) => continue,
        };
    }
}

pub fn process(stream: &mut TcpStream, thread_locked_table: Arc<RwLock<Vec<Mutex<Vec<(i32, i32)>>>>>, capacity: i32, command_str: String) -> i32{

    let command_units = command_str.split_whitespace().collect::<Vec<_>>();
    if command_units.len() < 2 {
        tcp_helper::write_string(stream, "0\n".to_owned());
        let error_value = "Server: Enter the command correctly".to_owned();
        tcp_helper::write_string(stream, error_value);
        return 1; 
    }

    let operation: &str = command_units[0];
    let key: i32 = convert_string_to_int(command_units[1].to_owned());

    if operation.eq("RESET") {
        reset(&thread_locked_table, capacity);
        return 0;
    }

    // GET
    if operation.eq("GET") {
        let result = get(&thread_locked_table, key);
        let error_code = match result {
            Ok(_) => "0\n".to_owned(),
            Err(_) => "1\n".to_owned(),
        };
        tcp_helper::write_string(stream, error_code);
        let value = match result {
            Ok(value) => value.to_string() + "\n",
            Err(_) => "Server: Not found\n".to_owned(),
        };
        tcp_helper::write_string(stream, value);
    }

    // PUT
    else if operation.eq("PUT"){
        let value = convert_string_to_int(command_units[2].to_owned());
        let error_code = match put(&thread_locked_table, key, value){
            Ok(_) => "0\n".to_owned(),
            Err(_) => "1\n".to_owned(),
        };
        tcp_helper::write_string(stream, error_code);
        tcp_helper::write_string(stream, "Server: PUT Succeeded\n".to_owned());
    }

    else {
        tcp_helper::write_string(stream, "1\n".to_owned());
        let error_code = "Server: FAILED - Wrong command\n".to_owned();
        tcp_helper::write_string(stream, error_code); 
    }
    return 1;
}