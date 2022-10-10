mod bench;
mod adapters;
mod hashmaps;

use bench::{generate_metrics, create_workloads};

use crate::adapters::striped::StripedHashMapTable;

fn main() {
    let no_of_threads = 4;
    // let csv_file = "sample_output.csv";
    let workloads = create_workloads(no_of_threads);
    generate_metrics::<StripedHashMapTable>("Striped lock".to_string(), workloads, no_of_threads);
}


