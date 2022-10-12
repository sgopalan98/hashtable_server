use std::io::{Read, Error, BufReader, BufRead};
use std::sync::{Arc, Mutex, RwLock};
use std::net::TcpStream;
use std::thread;
use std::io::Write;

use crate::hashmap::StripedHashTable;
use crate::tcp_helper;

fn convert_string_to_int(string: String) -> usize{
    return string.parse::<usize>().unwrap();
}

// fn reset(thread_locked_table: &Arc<RwLock<Vec<Mutex<Vec<(i32, i32)>>>>>, capacity: i32) {
//     let mut old_buckets = thread_locked_table.write().unwrap();
    
//     let mut buckets = vec![];
//     for _ in 0..capacity {
//         buckets.push(Mutex::new(Vec::new()));
//     }
    
//     *old_buckets = buckets;
// }

pub fn process(mut stream: TcpStream, thread_locked_table: Arc<StripedHashTable>) {
    let mut command_str = tcp_helper::read_command(&mut stream);
    while !command_str.eq("CLOSE") {
        let command_units = command_str.split_whitespace().collect::<Vec<_>>();
        if command_units.len() < 2 {
            tcp_helper::write_string(&mut stream, "0\n".to_owned());
            let error_value = "Server: Enter the command correctly".to_owned();
            tcp_helper::write_string(&mut stream, error_value);
            return; 
        }

        let operation: &str = command_units[0];
        let key: usize = convert_string_to_int(command_units[1].to_owned());

        // if operation.eq("RESET") {
        //     reset(&thread_locked_table, capacity);
        //     return;
        // }

        // GET
        if operation.eq("GET") {
            let result = thread_locked_table.get(key);
            let value = match result {
                Ok(value) => value.to_string() + "\n",
                Err(_) => "0\n".to_owned(),
            };
            tcp_helper::write_string(&mut stream, value);
        }

        // PUT
        else if operation.eq("PUT"){
            let value = convert_string_to_int(command_units[2].to_owned());
            let error_code = match thread_locked_table.insert_or_update(key, value){
                Ok(_) => "0\n".to_owned(),
                Err(_) => "1\n".to_owned(),
            };
            tcp_helper::write_string(&mut stream, error_code);
        }

        // REMOVE
        else if operation.eq("REMOVE"){
            let error_code = match thread_locked_table.remove(key){
                Ok(_) => "0\n".to_owned(),
                Err(_) => "1\n".to_owned(),
            };
            tcp_helper::write_string(&mut stream, error_code);
        }

        else {
            tcp_helper::write_string(&mut stream, "1\n".to_owned());
            let error_code = "Server: FAILED - Wrong command\n".to_owned();
            tcp_helper::write_string(&mut stream, error_code); 
        }
        command_str = tcp_helper::read_command(&mut stream);
    }
    return;
}