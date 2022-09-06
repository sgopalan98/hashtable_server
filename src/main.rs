mod server_thread_handler;
mod tcp_helper;
use std::sync::{Arc, Mutex};
use std::thread;
use std::net::TcpListener;

fn main() {
    // Create a hashtable
    let capacity = 10;
    let mut locked_striped_hashtable: Vec<Arc<Mutex<Vec<(i32, i32)>>>> = Vec::new();
    for _ in 0..capacity {
        locked_striped_hashtable.push(Arc::new(Mutex::new(Vec::new())));
    }
    // Get the address and open the port
    let address = "0.0.0.0:7878";
    let listener: TcpListener = TcpListener::bind(address).unwrap();
    for stream in listener.incoming() {
        let mut arc_striped_hashtable: Vec<Arc<Mutex<Vec<(i32, i32)>>>> = Vec::new();
        for i in 0..capacity {
            arc_striped_hashtable.push(Arc::clone(&locked_striped_hashtable[i]));
        }
        let stream = stream.unwrap();
        let t = thread::spawn(move|| {
            server_thread_handler::process(stream, arc_striped_hashtable);
        });
    }
}


