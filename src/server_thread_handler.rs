use std::sync::Arc;
use std::net::TcpStream;

use crate::{tcp_helper, striped_hash_table};

fn convert_string_to_int(string: String) -> i32{
    return string.parse::<i32>().unwrap();
}

pub fn process(mut stream: TcpStream, thread_locked_table: Arc<striped_hash_table::HashTable>){
    
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

    if operation.eq("GET") {
        let error_code = match thread_locked_table.get(key) {
            Ok(_) => "0\n".to_owned(),
            Err(_) => "1\n".to_owned(),
        };
        tcp_helper::write_string(&mut stream, error_code);

        let value = match thread_locked_table.get(key) {
            Ok(value) => value.to_string(),
            Err(_) => "Server: Not found".to_owned(),
        };
        tcp_helper::write_string(&mut stream, value);
    }

    else if operation.eq("PUT"){
        let value = convert_string_to_int(command_units[2].to_owned());
        let error_code = match thread_locked_table.put(key, value){
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
        return; 
    }
    println!("DONE for {}", command_str);

}