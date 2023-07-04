use std::sync::Arc;
use std::{io::Write};
use std::net::TcpStream;
use crate::{receive_request, KeyValueType};
use dashmap::DashMap;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
struct Request {
    operations: Vec<Operation>,
}

#[derive(Debug, Serialize, Deserialize)]
struct OperationResults {
    results: Vec<OperationResult>,
}

#[derive(Debug, Serialize, Deserialize)]
enum Operation {
    Read { key: KeyValueType },
    Insert { key: KeyValueType, value: KeyValueType },
    Remove { key: KeyValueType },
    Increment { key: KeyValueType },
    Close
}

#[derive(Debug, Serialize, Deserialize)]
enum OperationResult {
    ReadSuccess(KeyValueType),
    ReadFailure(String),
    InsertNew(KeyValueType),
    InsertOld(KeyValueType),
    RemoveSuccess(KeyValueType),
    RemoveFailure(String),
    IncrementSuccess(KeyValueType),
    IncrementFailure(String),
}

#[derive(Debug, Serialize, Deserialize)]
enum ResultData {
    String(String),
    Int(i32),
    // Add more types as needed
}

fn send_request<T: Serialize>(stream: &mut TcpStream, request: &T) {
    let request_json = serde_json::to_string(&request).expect("Failed to serialize request");
    stream.write_all(request_json.as_bytes()).expect("Failed to send request");
}

#[inline(never)]
pub fn process(mut stream: TcpStream, mut thread_locked_table: Arc<DashMap<KeyValueType, KeyValueType>>)
{
    loop {
        let request: Request = receive_request(&stream);
        let mut results = Vec::new();

        for operation in request.operations {
            let result = match operation {
                

                Operation::Read { key } => {
                    match thread_locked_table.get(&key) {
                        Some(value) => OperationResult::ReadSuccess(value.clone()),
                        None => OperationResult::ReadFailure(String::from("Key does not exist")),
                    }
                }


                Operation::Insert { key, value } => {
                    let old_value = thread_locked_table.insert(key.clone(), value.clone());
                    match old_value {
                        Some(old_value) => OperationResult::InsertOld(old_value),
                        None => OperationResult::InsertNew(value),
                    }
                }


                Operation::Remove { key } => {
                    match thread_locked_table.remove(&key) {
                        Some((key, value)) => OperationResult::RemoveSuccess(value.clone()),
                        None => OperationResult::RemoveFailure(String::from("Key does not exist")),
                    }
                }

                Operation::Increment { key } => {
                    match thread_locked_table.get_mut(&key) {
                        Some(entry) => {
                            match *entry {
                                KeyValueType::Int(mut value) => {
                                    value = value + 1;
                                    OperationResult::IncrementSuccess(KeyValueType::Int(value + 1))
                                },
                                KeyValueType::String(_) => OperationResult::IncrementFailure(String::from("Value is not an integer")),
                            }
                        }
                        None => OperationResult::IncrementFailure(String::from("Key does not exist")),
                    }
                }
                Operation::Close => return,
            };
            
            results.push(result);
        }
        let operation_results = OperationResults{
            results
        };
        send_request(&mut stream, &operation_results);
    }
}
