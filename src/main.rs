mod server_thread_handler;
mod tcp_helper;


use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::net::TcpListener;

fn main() {
    // Create a hashtable
    let capacity = 1000;
    let locked_table = Arc::new(Mutex::new(HashMap::with_capacity(capacity)));

    // Get the address and open the port
    let address = "0.0.0.0:7878";
    let listener: TcpListener = TcpListener::bind(address).unwrap();
    
    for stream in listener.incoming() {
        let thread_locked_table = Arc::clone(&locked_table);
        let stream = stream.unwrap();
        let _t = thread::spawn(move|| {
            server_thread_handler::handle(stream, thread_locked_table, capacity);
        });
    }
}


