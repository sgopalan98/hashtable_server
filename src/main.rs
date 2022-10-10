mod bench;
mod collections;
mod hashmaps;

use bench::{generate_metrics, create_workloads};

use crate::collections::striped::StripedHashMapTable;
use crate::collections::single::RwLockStdHashMapTable;

fn main() {
    let no_of_threads = 4;
    let workloads = create_workloads(no_of_threads);
    generate_metrics::<StripedHashMapTable>("Striped Lock".to_string(), workloads.clone(), no_of_threads);
    generate_metrics::<RwLockStdHashMapTable>("Single Lock".to_string(), workloads.clone(), no_of_threads);
}


