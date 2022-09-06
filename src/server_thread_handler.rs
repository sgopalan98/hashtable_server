use std::sync::{Arc, Mutex};
use std::net::TcpStream;
use std::thread;
use std::time::Duration;
use crate::tcp_helper;

fn convert_string_to_int(string: String) -> i32{
    return string.parse::<i32>().unwrap();
}

fn get(thread_locked_table: &Vec<Arc<Mutex<Vec<(i32, i32)>>>>, key: i32) -> Result<i32, i32> {
    let index = key as usize % thread_locked_table.len();
    let bucket = thread_locked_table[index].lock().unwrap();
    thread::sleep(Duration::from_millis(400));
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

fn put(thread_locked_table: &mut Vec<Arc<Mutex<Vec<(i32, i32)>>>>, key: i32, value: i32) -> Result<i32, i32> {
    let index = key as usize % thread_locked_table.len();
    let mut bucket = thread_locked_table[index].lock().unwrap();
    thread::sleep(Duration::from_millis(400));
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

pub fn process(mut stream: TcpStream, mut thread_locked_table: Vec<Arc<Mutex<Vec<(i32, i32)>>>>){
    
    let command_str = tcp_helper::read_command(&mut stream);
    let command_units = command_str.split_whitespace().collect::<Vec<_>>();
    
    if command_units.len() < 2 {
        tcp_helper::write_string(&mut stream, "0\n".to_owned());
        
        let error_value = "Server: Enter the command correctly".to_owned();
        tcp_helper::write_string(&mut stream, error_value);
        return; 
    }

    let operation: &str = command_units[0];
    let key: i32 = convert_string_to_int(command_units[1].to_owned());
    // println!("EXECUTING {}\n", command_str);
    if operation.eq("GET") {
        let error_code = match get(&thread_locked_table, key) {
            Ok(_) => "0\n".to_owned(),
            Err(_) => "1\n".to_owned(),
        };
        tcp_helper::write_string(&mut stream, error_code);

        let value = match get(&thread_locked_table, key) {
            Ok(value) => value.to_string(),
            Err(_) => "Server: Not found".to_owned(),
        };
        tcp_helper::write_string(&mut stream, value);
    }

    else if operation.eq("PUT"){
        let value = convert_string_to_int(command_units[2].to_owned());
        let error_code = match put(&mut thread_locked_table, key, value){
            Ok(_) => "0\n".to_owned(),
            Err(_) => "1\n".to_owned(),
        };
        tcp_helper::write_string(&mut stream, error_code);
        tcp_helper::write_string(&mut stream, "Server: PUT Succeeded\n".to_owned());
    }

    else {
        tcp_helper::write_string(&mut stream, "1\n".to_owned());
        
        let error_code = "Server: FAILED - Wrong command\n".to_owned();
        tcp_helper::write_string(&mut stream, error_code); 
    }
    // println!("FINISHED {}\n", command_str);
    return;
}