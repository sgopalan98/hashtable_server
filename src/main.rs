mod server_thread_handler;
mod tcp_helper;


use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::net::TcpListener;

fn main() {
    // Create a hashtable
    let locked_table = Arc::new(Mutex::new(HashMap::new()));

    // Get the address and open the port
    let address: String = std::env::args().nth(1).expect("No address given");
    let listener: TcpListener = TcpListener::bind(address).unwrap();
    
    for stream in listener.incoming() {
        let thread_locked_table = Arc::clone(&locked_table);
        let stream = stream.unwrap();
        let t = thread::spawn(move|| {
            server_thread_handler::process(stream, thread_locked_table);
        });
        t.join().expect("Thread failed to execute");
    }
}


