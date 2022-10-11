mod bench;
mod collections;
mod hashmaps;

use bench::generate_metrics;
use structopt::StructOpt;

use crate::collections::striped::StripedHashMapTable;
use crate::collections::single::RwLockStdHashMapTable;

#[derive(Debug, StructOpt)]
pub struct Options {
    #[structopt(short = "t", long = "threads", default_value = "4")]
    threads: u32
}

fn main() {
    let options = Options::from_args();
    println!("The command line arguments are {:?}", options);


    generate_metrics::<StripedHashMapTable>("StripedLock".to_string(), options.threads);
    generate_metrics::<RwLockStdHashMapTable>("SingleLock".to_string(), options.threads);
}


