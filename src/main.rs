mod adapters;
mod tcp_helper;
mod sharded_map_handler;
mod whole_map_handler;



use adapters::{DashMapAdapter, LeapMapAdapter, SingleLockMapAdapter, StripedHashMapAdapter};
use dashmap::DashMap;
use leapfrog::LeapMap;
use std::collections::HashMap;
use std::io::BufReader;
use std::net::{TcpListener, TcpStream};
use std::sync::Mutex;
use std::{thread, hash};
use std::{any::Any, sync::Arc};
use structopt::StructOpt;

pub trait Adapter {
    type Key: From<u64>;
    type Value: From<u64>;

    fn create_with_capacity(capacity: usize) -> Self;

    fn clone(&self) -> Self;

    /// Perform a lookup for `key`.
    ///
    /// Should return `true` if the key is found.
    fn get(&mut self, key: &Self::Key) -> bool;

    /// Insert `key` into the collection.
    ///
    /// Should return `true` if no value previously existed for the key.
    fn insert(&mut self, key: &Self::Key, value: Self::Value) -> bool;

    /// Remove `key` from the collection.
    ///
    /// Should return `true` if the key existed and was removed.
    fn remove(&mut self, key: &Self::Key) -> bool;

    /// Update the value for `key` in the collection, if it exists.
    ///
    /// Should return `true` if the key existed and was updated.
    ///
    /// Should **not** insert the key if it did not exist.
    fn update(&mut self, key: &Self::Key) -> bool;
}

fn convert_string_to_int(string: String) -> usize {
    let string = string.trim();
    let parsed_number = match string.parse::<usize>() {
        Ok(no) => no,
        Err(_) => 0,
    };
    return parsed_number;
}

#[derive(Debug, StructOpt)]
pub struct Options {
    #[structopt(short, long, default_value = "dashmap")]
    pub map: String,
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
    let mut ops_st = 0;
    let mut hash_map_type = "DashMap";

    for stream in listener.incoming().take(1) {
        let mut stream = match stream {
            Ok(tcp_stream) => tcp_stream,
            Err(_) => panic!("NO STREAM"),
        };
        let mut reader = BufReader::new(match stream.try_clone() {
            Ok(stream) => stream,
            Err(_) => panic!("Cannot clone stream"),
        });
        let command = tcp_helper::read_setup(&mut stream, &mut reader);
        let command_units = command.split_whitespace().collect::<Vec<_>>();
        let capacity_command = command_units[0].to_owned();
        let no_of_threads_command = command_units[1].to_owned();
        let ops_st_command = command_units[2].to_owned();
        println!("{} {} {} {}\n", capacity_command, no_of_threads_command, ops_st_command, hash_map_type);
        capacity = convert_string_to_int(capacity_command);
        no_of_threads = convert_string_to_int(no_of_threads_command);
        ops_st = convert_string_to_int(ops_st_command);
    }

    if hash_map_type.eq("SingleLock") {
        // Create a Map
        let map = Arc::new(Mutex::new(HashMap::with_capacity(capacity)));

        // Create worker threads - #said no of threads
        let mut threads = vec![];
        for stream in listener.incoming().take(no_of_threads * 2) {
            let thread_specific_hashtable = map.clone();
            let stream = match stream {
                Ok(stream) => stream,
                Err(_) => panic!("Cannot obtain stream"),
            };
            threads.push(thread::spawn(move || {
                whole_map_handler::process(stream, thread_specific_hashtable, ops_st);
            }));
        }

        // Wait for the threads to finish
        for thread in threads {
            thread.join();
        }
    }

    else if hash_map_type.eq("Striped") {
        // Create a Map
        let map = StripedHashMapAdapter::create_with_capacity(capacity);

                
        let mut prefiller_streams = vec![];
        for stream in listener.incoming().take(no_of_threads) {
            let stream = match stream {
                Ok(stream) => stream,
                Err(_) => panic!("Cannot clone stream"),
            };
            prefiller_streams.push(stream);
        }

        // Create worker threads - #said no of threads
        let mut prefiller_threads = vec![];
        for stream in prefiller_streams {
            let thread_specific_hashtable = map.clone();
            prefiller_threads.push(thread::spawn(move || {
                sharded_map_handler::process(stream, thread_specific_hashtable, ops_st);
            }));
        }

        // Wait for the threads to finish
        for thread in prefiller_threads {
            thread.join();
        }


        let mut work_streams = vec![];
        for stream in listener.incoming().take(no_of_threads) {
            let stream = match stream {
                Ok(stream) => stream,
                Err(_) => panic!("Cannot clone stream"),
            };
            work_streams.push(stream);
        }

        // Create worker threads - #said no of threads
        let mut work_threads = vec![];
        for stream in work_streams {
            let thread_specific_hashtable = map.clone();
            work_threads.push(thread::spawn(move || {
                sharded_map_handler::process(stream, thread_specific_hashtable, ops_st);
            }));
        }

        // Wait for the threads to finish
        for thread in work_threads {
            thread.join();
        }
    }

    else if hash_map_type.eq("DashMap") {
        // Create a Map
        let map = DashMapAdapter::create_with_capacity(1 << capacity);

        
        let mut prefiller_streams = vec![];
        for stream in listener.incoming().take(no_of_threads) {
            let stream = match stream {
                Ok(stream) => stream,
                Err(_) => panic!("Cannot clone stream"),
            };
            prefiller_streams.push(stream);
        }

        // Create worker threads - #said no of threads
        let mut prefiller_threads = vec![];
        for stream in prefiller_streams {
            let thread_specific_hashtable = map.clone();
            prefiller_threads.push(thread::spawn(move || {
                sharded_map_handler::process(stream, thread_specific_hashtable, ops_st);
            }));
        }

        // Wait for the threads to finish
        for thread in prefiller_threads {
            thread.join();
        }


        let mut work_streams = vec![];
        for stream in listener.incoming().take(no_of_threads) {
            let stream = match stream {
                Ok(stream) => stream,
                Err(_) => panic!("Cannot clone stream"),
            };
            work_streams.push(stream);
        }

        // Create worker threads - #said no of threads
        let mut work_threads = vec![];
        for stream in work_streams {
            let thread_specific_hashtable = map.clone();
            work_threads.push(thread::spawn(move || {
                sharded_map_handler::process(stream, thread_specific_hashtable, ops_st);
            }));
        }

        // Wait for the threads to finish
        for thread in work_threads {
            thread.join();
        }
    }

}
