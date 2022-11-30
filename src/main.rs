mod thread_handlers;
mod tcp_helper;
use std::io::Write;
use std::sync::Arc;
use std::thread;
use std::net::TcpListener;
use futures::executor::block_on;
use striped_hashmap::StripedHashMap;
use dashmap::DashMap;
use thread_handlers::dashmap_thread_handler;


use tonic::{transport::Server, Request, Response, Status};


use hashmap_server_mod::hash_map_server::{HashMap, HashMapServer};
use hashmap_server_mod::{HashMapRequest, HashMapReply};

pub mod hashmap_server_mod {
    tonic::include_proto!("hashmap");
}

#[derive(Debug, Default)]
pub struct ThreadLocalHashMap {
    dashmap: Arc<DashMap<u128, u128>>
}

#[tonic::async_trait]
impl HashMap for ThreadLocalHashMap {

    async fn get(
        &self,
        request: Request<HashMapRequest>,
    ) -> Result<Response<HashMapReply>, Status> {
        let key = request.into_inner().key;
        let result = self.dashmap.get(&(key as u128));
        let reply = HashMapReply { error_code: result.is_some() };

        Ok(Response::new(reply))
    }

    async fn insert(
        &self,
        request: Request<HashMapRequest>,
    ) -> Result<Response<HashMapReply>, Status> {
        let key = request.into_inner().key;
        let result = self.dashmap.insert(key as u128, 0);
        let reply = HashMapReply { error_code: result.is_none() };

        Ok(Response::new(reply))
    }

    async fn remove(
        &self,
        request: Request<HashMapRequest>,
    ) -> Result<Response<HashMapReply>, Status> {
        let key = request.into_inner().key;
        let result = self.dashmap.remove(&(key as u128));
        let reply = HashMapReply { error_code: result.is_some() };

        Ok(Response::new(reply))
    }


    async fn update(
        &self,
        request: Request<HashMapRequest>,
    ) -> Result<Response<HashMapReply>, Status> {
        let key = request.into_inner().key;
        let result = self.dashmap.get_mut(&(key as u128)).map(|mut v| *v += 1);
        let reply = HashMapReply { error_code: result.is_some() };

        Ok(Response::new(reply))
    }

    async fn reset(
        &self,
        request: Request<HashMapRequest>,
    ) -> Result<Response<HashMapReply>, Status> {
        self.dashmap.clear();
        let reply = HashMapReply { error_code: true };

        Ok(Response::new(reply))
    }

}

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() {
    let addr = "[::1]:50051";
    // Create a hashtable
    let capacity = 1000;
    let locked_dashmap: Arc<DashMap<u128, u128>> = Arc::new(DashMap::with_capacity(capacity));

    
    // Return the new port
    let thread_local_hashmap = ThreadLocalHashMap{ dashmap: locked_dashmap };

    block_on(Server::builder()
    .add_service(HashMapServer::new(thread_local_hashmap))
    .serve(addr.parse().unwrap()));
    // // Create a new thread and spawn the service at the new port.
    // // New service should implement all the functions.
    // tokio::runtime::Builder::new_multi_thread()
    // .enable_all()
    // .build()
    // .unwrap()
    // .block_on();
}


