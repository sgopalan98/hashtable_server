mod adapters;
mod tcp_helper;
use adapters::{DashMapAdapter, LeapMapAdapter, SingleLockMapAdapter};
use dashmap::DashMap;
use leapfrog::LeapMap;
use std::io::BufReader;
use std::net::TcpListener;
use std::thread;
use std::{any::Any, sync::Arc};
mod thread_map_handler;
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
    return string.parse::<usize>().unwrap();
}

#[derive(Debug, StructOpt)]
pub struct Options {
    #[structopt(short, long, default_value = "dashmap")]
    pub map: String,
}

fn main() -> ! {
    // Start server
    let address = "0.0.0.0:7879";
    let listener: TcpListener = TcpListener::bind(address).unwrap();
    let options = Options::from_args();

    // First connection should get capacity and no of threads
    loop {
        let mut capacity = 0;
        let mut no_of_threads = 0;

        for stream in listener.incoming().take(1) {
            let mut stream = stream.unwrap();
            let mut reader = BufReader::new(stream.try_clone().unwrap());
            let command = tcp_helper::read_setup(&mut stream, &mut reader);
            let command_units = command.split_whitespace().collect::<Vec<_>>();
            let capacity_command = command_units[0].to_owned();
            let no_of_threads_command = command_units[1].to_owned();
            println!("{} {}\n", capacity_command, no_of_threads_command);
            capacity = convert_string_to_int(capacity_command);
            no_of_threads = convert_string_to_int(no_of_threads_command);
        }

        // Create a Map
        let map = SingleLockMapAdapter::create_with_capacity(capacity);

        // Create worker threads - #said no of threads
        let mut threads = vec![];
        for stream in listener.incoming().take(no_of_threads * 2) {
            let thread_specific_hashtable = map.clone();
            let stream = stream.unwrap();
            threads.push(thread::spawn(move || {
                thread_map_handler::process(stream, thread_specific_hashtable);
            }));
        }

        // Wait for the threads to finish
        for thread in threads {
            thread.join().unwrap();
        }
    }
}
