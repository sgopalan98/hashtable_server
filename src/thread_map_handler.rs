use std::io::Write;
use std::{sync::Arc, io::BufReader};
use std::net::TcpStream;

use dashmap::DashMap;
use crate::{tcp_helper, Adapter};

fn convert_string_to_int(string: String) -> usize{
    return string.parse::<usize>().unwrap();
}

pub fn process<T>(mut stream: TcpStream, mut thread_locked_table: T) where T: Adapter<Key = u64, Value = u64> {
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    loop {
        let command_u8s = tcp_helper::read_command(&mut stream, &mut reader);
        
        if command_u8s.len() == 0 {
            println!("Not receiving anything");
            continue;
        }
        let mut error_codes = vec![0u8; 100];
        let mut done = 0;
        for index in 0..100 {
            let start_index = 9 * index;
            let end_index = 9 * index + 9;
            let operation = command_u8s[start_index];
            let key = u64::from_be_bytes(command_u8s[(start_index + 1)..end_index].try_into().unwrap());
            let mut error_code:u8 = 0;
            // CLOSE
            if operation == 0 {
                done = 1;
                break;
            }

            // GET
            else if operation == 1 {
                error_code = match thread_locked_table.get(&(key as u64)) {
                    true => 0,
                    false => 1,
                };
                error_codes[index] = error_code;
            }

            // INSERT
            else if operation == 2 {
                error_code = match thread_locked_table.insert(&(key as u64), 0 as u64){
                    true => 0,
                    false => 1,
                };
                error_codes[index] = error_code;
            }

            // REMOVE
            else if operation == 3 {
                error_code = match thread_locked_table.remove(&(key as u64)){
                    true => 0,
                    false => 1,
                };
                error_codes[index] = error_code;
            }

            // UPDATE
            else if operation == 4 {
                error_code = match thread_locked_table.update(&(key as u64)){
                    true => 0,
                    false => 1,
                };
                error_codes[index] = error_code;
            }

            else {
                
            }
        }
        stream.write(&error_codes);
        if done == 1 {
            return;
        }
    }
}