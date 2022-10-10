mod bench;
mod adapters;
mod hashmaps;

use bench::generate_metrics;

use crate::adapters::striped::StripedHashMapTable;

fn main() {
    let no_of_threads = 4;
    let csv_file = "sample_output.csv";
    generate_metrics::<StripedHashMapTable>(no_of_threads, csv_file.to_owned());
}


