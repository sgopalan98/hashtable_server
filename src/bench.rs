use bustle::*;
use std::{fmt::Debug, io};
use std::time::Duration;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Record {
    pub name: String,
    pub total_ops: u64,
    pub threads: u32,
    #[serde(with = "timestamp")]
    pub spent: Duration,
    pub throughput: f64,
    #[serde(with = "timestamp")]
    pub latency: Duration,
}

mod timestamp {
    use super::*;

    use serde::{de::Deserializer, ser::Serializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        u64::deserialize(deserializer).map(Duration::from_nanos)
    }

    pub fn serialize<S>(value: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (value.as_nanos() as u64).serialize(serializer)
    }
}

pub fn create_workloads(no_of_threads: u32) -> Vec<(String, Workload)> {
    let read_heavy_mix = Mix {
        read: 98,
        insert: 1,
        remove: 1,
        update: 0,
        upsert: 0,
    };
    let read_heavy_workload = *Workload::new(no_of_threads as usize, read_heavy_mix)
        .initial_capacity_log2(24)
        .prefill_fraction(0.8);

    let rapid_grow_mix = Mix {
        read: 5,
        insert: 80,
        remove: 5,
        update: 10,
        upsert: 0,
    };

    let rapid_grow_workload = *Workload::new(no_of_threads as usize, rapid_grow_mix)
        .initial_capacity_log2(24)
        .prefill_fraction(0.8);

    let exchange_mix = Mix {
        read: 10,
        insert: 40,
        remove: 40,
        update: 10,
        upsert: 0,
    };

    let exchange_workload = *Workload::new(no_of_threads as usize, exchange_mix)
        .initial_capacity_log2(24)
        .prefill_fraction(0.8);
    

    let mut workloads = vec![];
    workloads.push(("ReadHeavy.csv".to_owned(), read_heavy_workload));
    workloads.push(("RapidGrow.csv".to_owned(), rapid_grow_workload));
    workloads.push(("Exchange.csv".to_owned(), exchange_workload));
    return workloads;
}


pub fn generate_metrics<C>(collection_name: String, workloads:Vec<(String, Workload)>, no_of_threads: u32)
where 
C: Collection,
<C::Handle as CollectionHandle>:: Key: Send + Debug,
{
    // For every workload,
    for (name, workload) in workloads.into_iter() {
        // Run the workload and get measurement.
        let measurement = workload.run_silently::<C>();
        // Write to PATH
        let path = "Results/".to_owned() + collection_name.as_str() + name.as_str();
        let mut wr = csv::WriterBuilder::new()
            .from_path(path).unwrap();
        wr.serialize(Record{
            name: name,
            total_ops: measurement.total_ops,
            threads: no_of_threads,
            spent: measurement.spent,
            throughput: measurement.throughput,
            latency: measurement.latency,
            })
            .expect("cannot serialize");
        wr.flush().expect("cannot flush");
    }
}