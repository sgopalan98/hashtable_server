use bustle::*;
use std::fmt::Debug;

pub fn generate_metrics<C>(no_of_threads: u32, csv_file: String) -> Measurement 
where 
C: Collection,
<C::Handle as CollectionHandle>:: Key: Send + Debug,
{

    // Create a workload.
    let workload = create_workload(no_of_threads);
    // Run the workload and get measurement.
    let measurement = workload.run_silently::<C>();
    println!("{:?}", measurement);
    return measurement;
}

fn create_workload(no_of_threads: u32) -> Workload {
    // Read heavy workload
    let mix = Mix {
        read: 98,
        insert: 1,
        remove: 1,
        update: 0,
        upsert: 0,
    };

    *Workload::new(no_of_threads as usize, mix)
        .initial_capacity_log2(24)
        .prefill_fraction(0.8)
}