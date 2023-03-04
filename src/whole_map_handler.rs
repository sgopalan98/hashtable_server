use std::collections::HashMap;
use std::io::Write;
use std::net::TcpStream;
use std::sync::Mutex;
use std::{io::BufReader, sync::Arc};

use crate::{tcp_helper, Adapter};
use dashmap::DashMap;

fn convert_string_to_int(string: String) -> usize {
    return string.parse::<usize>().unwrap();
}

pub fn process(mut stream: TcpStream, mut thread_locked_table: Arc<Mutex<HashMap<u64, u64>>>, ops_st: usize)
{
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    loop {
        let command_u8s = tcp_helper::read_command(&mut stream, &mut reader, ops_st);

        if command_u8s.len() == 0 {
            println!("Not receiving anything");
            continue;
        }
        let mut hashtable = thread_locked_table.lock().unwrap();

        let mut error_codes = vec![0u8; ops_st];
        let mut done = 0;
        for index in 0..ops_st {
            let start_index = 9 * index;
            let end_index = 9 * index + 9;
            let operation = command_u8s[start_index];
            let key = u64::from_be_bytes(
                command_u8s[(start_index + 1)..end_index]
                    .try_into()
                    .unwrap(),
            );
            let mut error_code: u8 = 0;
            // CLOSE
            if operation == 0 {
                done = 1;
                break;
            }
            // GET
            else if operation == 1 {
                error_code = match hashtable.get(&(key as u64)).is_some() {
                    true => 0,
                    false => 1,
                };
                error_codes[index] = error_code;
            }
            // INSERT
            else if operation == 2 {
                error_code = match hashtable.insert(key as u64, 0 as u64).is_none() {
                    true => 0,
                    false => 1,
                };
                error_codes[index] = error_code;
            }
            // REMOVE
            else if operation == 3 {
                error_code = match hashtable.remove(&(key as u64)).is_some() {
                    true => 0,
                    false => 1,
                };
                error_codes[index] = error_code;
            }
            // UPDATE
            else if operation == 4 {
                error_code = match hashtable.get_mut(&key).map(|mut v| *v += 1).is_some() {
                    true => 0,
                    false => 1,
                };
                error_codes[index] = error_code;
            } else {
            }
        }
        drop(hashtable);
        stream.write(&error_codes);
        if done == 1 {
            return;
        }
    }
}
