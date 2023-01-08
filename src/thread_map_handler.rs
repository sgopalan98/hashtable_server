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
        let command_str = tcp_helper::read_command(&mut stream, &mut reader);
        let mut results = vec![];
        for command_str in command_str.split(",") {
            let command_units = command_str.split_whitespace().collect::<Vec<_>>();
            if command_str.len() == 0 {
                println!("Not receiving anything");
                continue;
            }

            let operation: &str = command_units[0];
            let key: usize = convert_string_to_int(command_units[1].to_owned());

            // CLOSE
            if operation.eq("CLOSE") {
                return;
            }

            // GET
            else if operation.eq("GET") {
                let result = thread_locked_table.get(&(key as u64));
                let error_code = match result {
                    true => "0".to_owned(),
                    false => "1".to_owned(),
                };
                results.push(error_code);
            }

            // PUT
            else if operation.eq("INSERT"){
                let error_code = match thread_locked_table.insert(&(key as u64), 0 as u64){
                    true => "0".to_owned(),
                    false => "1".to_owned(),
                };
                results.push(error_code);
            }

            // REMOVE
            else if operation.eq("REMOVE"){
                let error_code = match thread_locked_table.remove(&(key as u64)){
                    true => "0".to_owned(),
                    false => "1".to_owned(),
                };
                results.push(error_code);
            }

            // UPDATE
            else if operation.eq("UPDATE"){
                let error_code = match thread_locked_table.update(&(key as u64)){
                    true => "0".to_owned(),
                    false => "1".to_owned(),
                };
                results.push(error_code);
            }

            else {
                tcp_helper::write_string(&mut stream, "1\n".to_owned());
                let error_code = "Server: FAILED - Wrong command\n".to_owned();
                tcp_helper::write_string(&mut stream, error_code); 
            }
        }
        let result_str = format!("{}\n", results.join(","));
        tcp_helper::write_string(&mut stream, result_str); 
    }
}