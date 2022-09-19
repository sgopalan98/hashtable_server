use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::net::TcpStream;
use std::time::Instant;

use crate::tcp_helper;


pub fn process(mut stream: TcpStream, thread_locked_table: Arc<Mutex<HashMap<String, String>>>){
    
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
        {
            let key: String = command_units[1].to_owned();
            let mut thread_table = thread_locked_table.lock().unwrap();
            if operation.eq("GET") {
                let now = Instant::now();
                let error_code = match thread_table.get(&key) {
                    Some(_) => "0\n".to_owned(),
                    None => "1\n".to_owned(),
                };
                println!("{}\n", now.elapsed().as_micros());
                tcp_helper::write_string(&mut stream, error_code);

                let now = Instant::now();
                let value = match thread_table.get(&key) {
                    Some(value) => value.to_string() + "\n",
                    None => "Server: Not found\n".to_owned(),
                };
                println!("{}\n", now.elapsed().as_micros());
                tcp_helper::write_string(&mut stream, value);
            }

            else if operation.eq("PUT"){
                let value = command_units[2].to_owned();
                let now = Instant::now();
                thread_table.insert(key, value);
                println!("{}\n", now.elapsed().as_micros());
                tcp_helper::write_string(&mut stream, "0\n".to_owned());
                tcp_helper::write_string(&mut stream, "Server: PUT Succeeded\n".to_owned());
            }

            else {
                tcp_helper::write_string(&mut stream, "1\n".to_owned());
                let error_code = "Server: FAILED - Wrong command\n".to_owned();
                tcp_helper::write_string(&mut stream, error_code);
            }
            drop(thread_table);
        }
        command_str = tcp_helper::read_command(&mut stream);
    }
    return; 

}