use std::sync::Arc;
use std::net::TcpStream;

use dashmap::DashMap;
use crate::{tcp_helper, Adapter};

fn convert_string_to_int(string: String) -> usize{
    return string.parse::<usize>().unwrap();
}

pub fn process<T>(mut stream: TcpStream, mut thread_locked_table: T) where T: Adapter<Key = u64, Value = u64> {
    
    loop {
        let command_str = tcp_helper::read_command(&mut stream);
        let command_units = command_str.split_whitespace().collect::<Vec<_>>();
        if command_str.len() == 0 {
            continue;
        }

        let operation: &str = command_units[0];
        let key: usize = convert_string_to_int(command_units[1].to_owned());

        // CLOSE
        if operation.eq("CLOSE") {
            break;
        }

        // GET
        else if operation.eq("GET") {
            let result = thread_locked_table.get(&(key as u64));
            let error_code = match result {
                true => "0\n".to_owned(),
                false => "1\n".to_owned(),
            };
            tcp_helper::write_string(&mut stream, error_code);
        }

        // PUT
        else if operation.eq("INSERT"){
            let value = convert_string_to_int(command_units[2].to_owned());
            let error_code = match thread_locked_table.insert(&(key as u64), value as u64){
                true => "0\n".to_owned(),
                false => "1\n".to_owned(),
            };
            tcp_helper::write_string(&mut stream, error_code);
        }

        // REMOVE
        else if operation.eq("REMOVE"){
            let error_code = match thread_locked_table.remove(&(key as u64)){
                true => "0\n".to_owned(),
                false => "1\n".to_owned(),
            };
            tcp_helper::write_string(&mut stream, error_code);
        }

        // UPDATE
        else if operation.eq("UPDATE"){
            let error_code = match thread_locked_table.update(&(key as u64)){
                true => "0\n".to_owned(),
                false => "1\n".to_owned(),
            };
            tcp_helper::write_string(&mut stream, error_code);
        }

        else {
            tcp_helper::write_string(&mut stream, "1\n".to_owned());
            let error_code = "Server: FAILED - Wrong command\n".to_owned();
            tcp_helper::write_string(&mut stream, error_code); 
        }
    }
}