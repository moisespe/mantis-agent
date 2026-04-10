use axum::{routing::get, Router, Json};
use serde::Serialize;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::collector::system;
use crate::collector::process;
use crate::collector::disk;
use crate::collector::network;



#[derive(Serialize)]
pub struct SystemCore {
    cpu             : f32,
    memory_used     : u64,
    memory_total    : u64,
    timestamp       : u64,
}

#[derive(Serialize)]
pub struct ProcessCore {
    pid     : u32,
    name    : String,
    cpu     : f32,
    memory  : u64,
}

#[derive(Serialize)]
pub struct HealthCore {
    status      : String,
    timestamp   : u64,
}

#[derive(Serialize)]
pub struct DiskCore {
    name        : String,
    total       : u64,
    available   : u64,
}

#[derive(Serialize)]
pub struct NetworkCore {
    name        : String,
    received    : u64,
    transmitted : u64,
}

#[derive(Serialize)]
pub struct ConnectionCore{
    pub local_ip: String,
    pub local_port: u16,
}


async fn system() -> Json<SystemCore> {
    let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap().as_secs();

    let (cpu, used_mem, total_mem) = system::get_metrics();


    Json( SystemCore  {
        cpu             : cpu * 100.0,
        memory_used     : used_mem / 1024 / 1024,
        memory_total    : total_mem / 1024 / 1024,
        timestamp       : timestamp,
    })
}


async fn health() -> Json<HealthCore> {
    Json(HealthCore {
        status: "online".into(),
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap().as_secs(),
    })
}


async fn process() -> Json<Vec<ProcessCore>> {
    let process = process::get_process()
        .into_iter()
        .map(|(pid, name, cpu, memory)| ProcessCore {
            pid     : pid,
            name    : name,
            cpu     : cpu,
            memory  : memory / 1024 / 1024,
        })
        .collect();

    Json(process)
}


async fn disk() -> Json<Vec<DiskCore>> {

    let disks = disk::get_disks()
        .into_iter()
        .map(|(name, total, available)| DiskCore {
            name        : name,
            total       : total / 1024 / 1024,
            available   : available / 1024 / 1024,
        }).collect();

    Json(disks)
}


async fn network() -> Json<Vec<NetworkCore>> {
    let networks: Vec<NetworkCore> = network::get_network()
        .into_iter()
        .map(|(name, received, transmitted)| NetworkCore {
            name        : name,
            received    : received,
            transmitted : transmitted,
        })
        .collect();

    Json(networks)
}

async fn connections() ->Json<Vec<ConnectionCore>> { 
    let conns = network::get_connections();
    Json(conns)
}



pub fn create_routes() -> Router {
    Router::new()
            .route("/api/metrics/system", get(system))
            .route("/api/metrics/health", get(health))
            .route("/api/metrics/process", get(process))
            .route("/api/metrics/disk", get(disk))
            .route("/api/metrics/network", get(network))
            .route("/api/metrics/network/connections",get(connections))
}

