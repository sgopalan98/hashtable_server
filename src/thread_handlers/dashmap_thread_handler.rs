use std::io::Write;
use std::sync::Arc;
use std::net::TcpStream;

use dashmap::DashMap;
use crate::tcp_helper;

fn convert_string_to_int(string: String) -> usize{
    return string.parse::<usize>().unwrap();
}

pub fn process(mut stream: TcpStream, thread_locked_table: Arc<DashMap<u128, u128>>) {
    
    loop {
        let command_str = tcp_helper::read_command(&mut stream);
        let command_units = command_str.split_whitespace().collect::<Vec<_>>();
        if command_str.len() == 0 {
            continue;
        }

        let operation: &str = command_units[0];
        let key: usize = convert_string_to_int(command_units[1].to_owned());
        
        // RESET 
        if operation.eq("RESET") {
            println!("{}\n", command_str);
            thread_locked_table.clear();
            return;
        }

        if operation.eq("FINISH") {
            println!("{}\n", command_str);
            return;
        }

        // GET
        else if operation.eq("GET") {
            let result = thread_locked_table.get(&(key as u128));
            let error_code = match result {
                Some(value) => "0\n".to_owned(),
                None => "1\n".to_owned(),
            };
            tcp_helper::write_string(&mut stream, error_code);
        }

        // PUT
        else if operation.eq("INSERT"){
            let value = convert_string_to_int(command_units[2].to_owned());
            let error_code = match thread_locked_table.insert(key as u128, value as u128){
                Some(value) => "1\n".to_owned(),
                None => "0\n".to_owned(),
            };
            tcp_helper::write_string(&mut stream, error_code);
        }

        // REMOVE
        else if operation.eq("REMOVE"){
            let error_code = match thread_locked_table.remove(&(key as u128)){
                Some(_) => "0\n".to_owned(),
                None => "1\n".to_owned(),
            };
            tcp_helper::write_string(&mut stream, error_code);
        }

        // UPDATE
        else if operation.eq("UPDATE"){
            let error_code = match thread_locked_table.get_mut(&(key as u128)).map(|mut v| *v += 1){
                Some(_) => "0\n".to_owned(),
                None => "1\n".to_owned(),
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