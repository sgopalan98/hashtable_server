use std::io::{Read, Error, BufReader, BufRead};
use std::sync::{Arc, Mutex, RwLock};
use std::net::TcpStream;
use std::thread;
use std::io::Write;

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

fn write_string(locked_stream: &Arc<Mutex<TcpStream>>, output: String) {
    let mut stream = locked_stream.lock().unwrap();
    stream.write(output.as_bytes()).unwrap();
}

pub fn handle(stream: TcpStream, thread_locked_table: Arc<RwLock<Vec<Mutex<Vec<(i32, i32)>>>>>, capacity: i32) {
    read_command(stream, thread_locked_table, capacity)
}

fn read_command(stream: TcpStream, thread_locked_table: Arc<RwLock<Vec<Mutex<Vec<(i32, i32)>>>>>, capacity: i32){
    let stream_clone = stream.try_clone().unwrap();
    let mut reader = BufReader::new(stream_clone);
    let arc_stream = Arc::new(Mutex::new(stream));
    loop {
        let mut input = String::new();
        let arc_cloned_stream = Arc::clone(&arc_stream);
        let result: Result<usize, Error>;
        {            
            result = reader.read_line(&mut input);
        }
        match result {
            Ok(_) => {
                if input.trim().eq("CLOSE") {
                    println!("CLOSING");
                    write_string(&arc_cloned_stream, input);
                    return;
                }
                if input.len() == 0 {
                    continue;
                }
                let table = Arc::clone(&thread_locked_table);
                thread::spawn(move || process(arc_cloned_stream, table, capacity, input));
            },
            Err(_) => continue,
        };
    }
}

pub fn process(stream: Arc<Mutex<TcpStream>>, thread_locked_table: Arc<RwLock<Vec<Mutex<Vec<(i32, i32)>>>>>, capacity: i32, command_str: String) -> i32{
    let command_units = command_str.split_whitespace().collect::<Vec<_>>();
    if command_units.len() < 2 {
        let error_value = "Server: Enter the command correctly".to_owned();
        write_string(&stream, error_value);
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
        let value = match result {
            Ok(value) => value.to_string(),
            Err(_) => "Server: Not found".to_owned(),
        };
        let return_value = format!("{};{}\n", command_str.trim(), value);
        write_string(&stream, return_value);
    }

    // PUT
    else if operation.eq("PUT"){
        let value = convert_string_to_int(command_units[2].to_owned());
        let error_code = match put(&thread_locked_table, key, value){
            Ok(_) => "0".to_owned(),
            Err(_) => "1".to_owned(),
        };
        let return_value = format!("{};{}\n", command_str.trim(), error_code);
        write_string(&stream, return_value);
    }

    else {
        let error_code = "Server: FAILED - Wrong command\n".to_owned();
        write_string(&stream, error_code); 
    }
    return 1;
}