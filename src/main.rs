mod server_thread_handler;
mod hashmap;
mod tcp_helper;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::net::TcpListener;

use hashmap::StripedHashTable;

fn main() {
    // Create a hashtable
    let capacity = 1000;
    let locked_striped_hashtable: Arc<StripedHashTable> = Arc::new(StripedHashTable::with_capacity(capacity));
    // Get the address and open the port
    let address = "0.0.0.0:7879";
    let listener: TcpListener = TcpListener::bind(address).unwrap();
    for stream in listener.incoming() {
        let thread_specific_hashtable = Arc::clone(&locked_striped_hashtable);
        let stream = stream.unwrap();
        thread::spawn(move|| {
            server_thread_handler::process(stream, thread_specific_hashtable);
        });
    }
}


