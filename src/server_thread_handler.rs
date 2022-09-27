use std::collections::HashMap;
use std::io::{BufReader, Error, BufRead, Write};
use std::sync::{Arc, Mutex};
use std::net::TcpStream;
use std::thread;

fn write_string(locked_stream: &Arc<Mutex<TcpStream>>, output: String) {
    let mut stream = locked_stream.lock().unwrap();
    stream.write(output.as_bytes());
}


pub fn handle(mut stream: TcpStream, thread_locked_table: Arc<Mutex<HashMap<String, String>>>, capacity: usize) {
    read_command(stream, thread_locked_table, capacity);
}

fn read_command(stream: TcpStream, thread_locked_table: Arc<Mutex<HashMap<String, String>>>, capacity: usize){
    // Setting as non blocking
    // stream.set_nonblocking(true).expect("Setting non blocking failed ");
    let stream_clone = stream.try_clone().unwrap();
    let mut reader = BufReader::new(stream_clone);
    let arc_stream = Arc::new(Mutex::new(stream));
    loop {
        let mut input = String::new();
        let arc_cloned_stream = Arc::clone(&arc_stream);
        let result: Result<usize, Error>;
        {            
            result = reader.read_line(&mut input);
        }
        match result {
            Ok(_) => {
                if input.trim().eq("CLOSE") {
                    println!("CLOSING");
                    write_string(&arc_cloned_stream, input);
                    return;
                }
                if input.len() == 0 {
                    continue;
                }
                let table = Arc::clone(&thread_locked_table);
                thread::spawn(move || process(arc_cloned_stream, table, capacity, input));
            },
            Err(_) => continue,
        };
    }
}


pub fn process(stream: Arc<Mutex<TcpStream>>, thread_locked_table: Arc<Mutex<HashMap<String, String>>>, capacity: usize, command_str: String){
    
    let command_units = command_str.split_whitespace().collect::<Vec<_>>();
    if command_units.len() < 2 {
        let error_value = "Server: Enter the command correctly".to_owned();
        write_string(&stream, error_value);
        return; 
    }

    let operation: &str = command_units[0];
    if operation.eq("RESET") {
        let mut thread_table = thread_locked_table.lock().unwrap();
        thread_table.clear();
        return;
    }

    {
        let key: String = command_units[1].to_owned();
        let mut thread_table = thread_locked_table.lock().unwrap();
        if operation.eq("GET") {
            let result = thread_table.get(&key);
            let value = match result {
                Some(value) => value.to_string(),
                None => "Server: Not found".to_owned(),
            };
            let return_value = format!("{};{}\n", command_str.trim(), value);
            write_string(&stream, return_value);
        }

        else if operation.eq("PUT"){
            let value = command_units[2].to_owned();
            thread_table.insert(key, value);
            let return_value = format!("{};{}\n", command_str.trim(), 1);
            write_string(&stream, return_value);
        }

        else {
            let error_code = "Server: FAILED - Wrong command\n".to_owned();
            write_string(&stream, error_code);
        }
        drop(thread_table);
    }
    return; 

}