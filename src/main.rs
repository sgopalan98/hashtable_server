mod thread_handlers;
mod tcp_helper;
use std::sync::Arc;
use std::thread;
use std::net::TcpListener;
use striped_hashmap::StripedHashMap;
use dashmap::DashMap;
use thread_handlers::dashmap_thread_handler;


fn main() {
    // Create a hashtable
    let capacity = 1000;
    // let locked_striped_hashtable: Arc<StripedHashMap> = Arc::new(StripedHashMap::with_capacity(capacity));
    let locked_dashmap: Arc<DashMap<u128, u128>> = Arc::new(DashMap::with_capacity(capacity));
    // Get the address and open the port
    let address = "0.0.0.0:7879";
    let listener: TcpListener = TcpListener::bind(address).unwrap();
    // for stream in listener.incoming() {
    //     let thread_specific_hashtable = Arc::clone(&locked_striped_hashtable);
    //     let stream = stream.unwrap();
    //     thread::spawn(move|| {
    //         server_thread_handler::process(stream, thread_specific_hashtable);
    //     });
    // }

    for stream in listener.incoming() {
        let thread_specific_hashtable = Arc::clone(&locked_dashmap);
        let stream = stream.unwrap();
        thread::spawn(move|| {
            dashmap_thread_handler::process(stream, thread_specific_hashtable);
        });
    }
}


