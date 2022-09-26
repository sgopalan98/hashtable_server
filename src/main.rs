mod server_thread_handler;
mod tcp_helper;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::net::TcpListener;

fn main() {
    // Create a hashtable
    let capacity = 1000;
    let locked_striped_hashtable: Arc<RwLock<Vec<Mutex<Vec<(i32, i32)>>>>> = Arc::new(RwLock::new(Vec::new()));
    {
        let mut buckets = locked_striped_hashtable.write().unwrap();
        for _ in 0..capacity {
            buckets.push(Mutex::new(Vec::new()));
        }
    }
    // Get the address and open the port
    let address = "0.0.0.0:7879";
    let listener: TcpListener = TcpListener::bind(address).unwrap();
    for stream in listener.incoming() {
        let thread_specific_hashtable = Arc::clone(&locked_striped_hashtable);
        let stream = stream.unwrap();
        thread::spawn(move|| {
            server_thread_handler::handle(stream, thread_specific_hashtable, capacity);
        });
    }
}


