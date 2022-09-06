mod server_thread_handler;
mod tcp_helper;
mod striped_hash_table;
use std::sync::Arc;
use std::thread;
use std::net::TcpListener;

fn main() {
    // Create a hashtable
    let locked_striped_hashtable = Arc::new(striped_hash_table::HashTable::new(10)); 

    // Get the address and open the port
    let address: String = std::env::args().nth(1).expect("No address given");
    let listener: TcpListener = TcpListener::bind(address).unwrap();
    
    for stream in listener.incoming() {
        let thread_locked_table = Arc::clone(&locked_striped_hashtable);
        let stream = stream.unwrap();
        let t = thread::spawn(move|| {
            server_thread_handler::process(stream, thread_locked_table);
        });
        t.join().expect("Thread failed to execute");
    }
}


