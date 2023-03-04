use std::io::Write;
use std::net::TcpStream;
use std::{io::BufReader, sync::Arc};

use crate::{tcp_helper, Adapter};

pub fn process<T>(mut stream: TcpStream, mut thread_locked_table: T, ops_st: usize)
where
    T: Adapter<Key = u64, Value = u64>,
{
    let mut reader = BufReader::new(match stream.try_clone() {
        Ok(stream) => stream,
        Err(_) => panic!("Cannot clone stream"),
    });
    loop {
        let command_u8s = tcp_helper::read_command(&mut stream, &mut reader, ops_st);

        if command_u8s.len() == 0 {
            println!("Not receiving anything");
            continue;
        }
        let mut error_codes = vec![0u8; ops_st];
        let mut done = 0;
        for index in 0..ops_st {
            let start_index = 9 * index;
            let end_index = 9 * index + 9;
            let operation = command_u8s[start_index];
            let key_u8s = &command_u8s[(start_index + 1)..end_index];
            let key = u64::from_be_bytes(
                match key_u8s.try_into() {
                    Ok(key) => key,
                    Err(_) => panic!("Cannot convert slice to array"),
                },
            );
            let error_code: u8;
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
                error_code = match thread_locked_table.insert(&(key as u64), 0 as u64) {
                    true => 0,
                    false => 1,
                };
                error_codes[index] = error_code;
            }
            // REMOVE
            else if operation == 3 {
                error_code = match thread_locked_table.remove(&(key as u64)) {
                    true => 0,
                    false => 1,
                };
                error_codes[index] = error_code;
            }
            // UPDATE
            else if operation == 4 {
                error_code = match thread_locked_table.update(&(key as u64)) {
                    true => 0,
                    false => 1,
                };
                error_codes[index] = error_code;
            } else {
            }
        }
        stream.write(&error_codes);
        if done == 1 {
            return;
        }
    }
}
