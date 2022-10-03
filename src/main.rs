mod server_thread_handler;
mod tcp_helper;


use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::{thread, env};
use std::time::{Instant, Duration};
use rand::seq::SliceRandom;
use rand_chacha::ChaChaRng;
use rand_chacha::rand_core::SeedableRng;


fn evaluate_hashtable(hashtable: Arc<Mutex<HashMap<usize, usize>>>, no_of_threads: usize, input_vector: Vec<usize>, get_per_put: i32) -> Duration {
    let iterations = 10;
    let no_of_items = input_vector.len();
    let items_per_thread = no_of_items / no_of_threads;
    let items_per_iteration = items_per_thread / iterations;
    let mut threads = vec![];
    let now = Instant::now();
    let arc_input = Arc::new(input_vector);
    for thread_no in 0..no_of_threads {
        let input = Arc::clone(&arc_input); 
        let cloned_hashtable = Arc::clone(&hashtable);
        threads.push(thread::spawn(move || {
            for iteration in 0..iterations {

                let base = thread_no * items_per_thread + iteration * items_per_iteration;
                let end = base + items_per_iteration;
                let thread_input = &input[base..end]; 
                
                // PUT
                for key in thread_input {
                    let mut thread_table = cloned_hashtable.lock().unwrap();
                    thread_table.insert(*key, *key);
                    drop(thread_table);
                }

                
                // GET
                for _ in 0..get_per_put {
                    for key in thread_input {
                        let mut thread_table = cloned_hashtable.lock().unwrap();
                        thread_table.get(&key);
                        drop(thread_table);
                    }
                }
                // println!("For thread {} iteration {} completed\n", thread_no, iteration);
            }
        }));
    }
    
    for thread in threads {
        // Wait for the thread to finish. Returns a result.
        let _ = thread.join();
    }

    let elapsed = now.elapsed();
    return elapsed;
}




fn main() {
    //Command line arguments
    let args: Vec<String> = env::args().collect();
    let no_of_threads_string = &args[1];
    println!("{}", no_of_threads_string);
    let no_of_threads = no_of_threads_string.parse().unwrap(); // No of hyperthreads
    // Create a hashtable
    let capacity = 1000;
    let locked_table = Arc::new(Mutex::new(HashMap::with_capacity(capacity)));


    let no_of_items: usize = 100000;
    
    // Input
    let base = 0;
    let end = no_of_items - 1;
    let mut input: Vec<_> = (base..=end).collect();
    // let mut rng = rand::thread_rng();
    let seed = [0; 32];
    let mut rng = ChaChaRng::from_seed(seed);

    input.shuffle(&mut rng);
    println!("The first few elements are {:?}", &input[0..5]);

    let mut elapsed_duration = vec![];

    let get_per_puts: Vec<i32> = (1..=5).collect();
    for get_per_put in get_per_puts.clone() {
        elapsed_duration.push(evaluate_hashtable(locked_table.clone(), no_of_threads, input.clone(), get_per_put));
    }
    
    let mut throughput_values = vec![];
    for (index, duration) in elapsed_duration.iter().enumerate() {
        let no_of_operations = no_of_items + no_of_items * (index + 1);
        println!("TIME TAKEN {:?}", duration.as_micros());
        let throughput = no_of_operations as f64 / duration.as_micros() as f64;
        println!("THROUGHPUT {}", throughput);
        // Append through put values
        throughput_values.push((100.0 / (get_per_puts.clone()[index] + 1) as f64, throughput));
        println!("THE % put is {}", 100.0 / (get_per_puts.clone()[index] + 1) as f64);
    	println!();
    }
}


