mod thread_handlers;
mod tcp_helper;
use std::sync::Arc;
use std::thread;
use std::net::TcpListener;
use dashmap::DashMap;
use thread_handlers::dashmap_thread_handler;

fn convert_string_to_int(string: String) -> usize{
    let string = string.trim();
    return string.parse::<usize>().unwrap();
}

fn main() {
    // Start server 
    let address = "0.0.0.0:7879";
    let listener : TcpListener = TcpListener::bind(address).unwrap();

    // First connection should get capacity and no of threads
    loop {
        let mut capacity = 0;
        let mut no_of_threads = 0;

        for stream in listener.incoming().take(1) {
            let mut stream = stream.unwrap();
            let command = tcp_helper::read_command(&mut stream);
            let command_units = command.split_whitespace().collect::<Vec<_>>();
            let capacity_command = command_units[0].to_owned();
            let no_of_threads_command = command_units[1].to_owned();
            println!("{} {}\n", capacity_command, no_of_threads_command);
            capacity = convert_string_to_int(capacity_command);
            no_of_threads = convert_string_to_int(no_of_threads_command);
        }
        // Create a hashtable with that capacity
        let dashmap = DashMap::with_capacity(capacity);
        let locked_dashmap: Arc<DashMap<u128, u128>> = Arc::new(dashmap);

        // Create worker threads - #said no of threads
        let mut threads = vec![];
        for stream in listener.incoming().take(no_of_threads) {
            let thread_specific_hashtable = Arc::clone(&locked_dashmap);
            let stream = stream.unwrap();
            threads.push(thread::spawn(move|| {
                dashmap_thread_handler::process(stream, thread_specific_hashtable);
            }));
        }

        // Wait for the threads to finish
        for thread in threads {
            thread.join().unwrap();
        }
    }
}


