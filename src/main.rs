mod sharded_map_handler;


use dashmap::DashMap;
use leapfrog::LeapMap;
use std::collections::HashMap;
use std::io::{BufReader, Read};
use std::net::{TcpListener, TcpStream};
use std::sync::Mutex;
use std::{thread, hash};
use std::{any::Any, sync::Arc};
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Options {
    #[structopt(short, long, default_value = "DashMap")]
    pub map: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct HandShakeRequest {
    client_threads: usize,
    server_threads: usize,
    ops_per_req: usize,
    capacity: usize,
    key_type: KeyValueType,
    value_type: KeyValueType
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[derive(Eq, Hash, PartialEq)]
pub enum KeyValueType {
    Int(u64),
    String(String),
    // Add more types as needed
}

fn receive_request<T: DeserializeOwned>(mut stream: &TcpStream) -> T {
    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer).unwrap();
    let request_json = String::from_utf8_lossy(&buffer[..bytes_read]).into_owned();
    // println!("request json: {}", request_json);
    let result = serde_json::from_str(&request_json).unwrap();
    // println!("done");
    return result;
}


fn main() {
    // Start server
    println!("starting server");
    let address = "0.0.0.0:7879";
    let listener: TcpListener = match TcpListener::bind(address) {
        Ok(listener) => listener,
        Err(_) => panic!("Cannot bind"),
    };
    let options = Options::from_args();

    
    let mut capacity = 0;
    let mut no_of_threads = 0;
    let mut ops_per_req = 0;
    let hash_map_type = "DashMap";
    let mut key_type: KeyValueType;
    let mut value_type: KeyValueType;

    for stream in listener.incoming().take(1) {
        let mut stream = match stream {
            Ok(tcp_stream) => tcp_stream,
            Err(_) => panic!("NO STREAM"),
        };
        let handshake_request: HandShakeRequest = receive_request(&stream);
        capacity = handshake_request.capacity;
        no_of_threads = handshake_request.client_threads;
        ops_per_req = handshake_request.ops_per_req;
        key_type = handshake_request.key_type;
        value_type = handshake_request.value_type;
    }

    if hash_map_type.eq("DashMap") {
        // Create a Map
        let map: Arc<DashMap<KeyValueType, KeyValueType>> = Arc::new(DashMap::with_capacity(1 << capacity));

        
        let mut worker_threads = vec![];
        for stream in listener.incoming().take(no_of_threads * 2) {
            let stream = match stream {
                Ok(stream) => stream,
                Err(_) => panic!("Cannot clone stream"),
            };
            let thread_specific_hashtable = Arc::clone(&map);
            worker_threads.push(thread::spawn(move || {
                sharded_map_handler::process(stream, thread_specific_hashtable);
            }));
        }
    
        // Wait for the threads to finish
        for thread in worker_threads {
            thread.join().unwrap();
        }
    }

}
