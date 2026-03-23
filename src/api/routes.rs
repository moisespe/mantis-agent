use axum::{routing::get, Router, Json};
use serde::Serialize;
use crate::collector::system;
use std::time::{SystemTime, UNIX_EPOCH};


#[derive(Serialize)]
pub struct CoreSystem{
    core: CoreMetrics,
    process: Option<Vec<ProcessInfo>>,
}

#[derive(Serialize)]
pub struct CoreMetrics {
    cpu             : f32,
    memory_used     : u64,
    memory_total    : u64,
    timestamp       : u64,
}

#[derive(Serialize)]
pub struct ProcessInfo {
    pid     : u32,
    name    : String,
    cpu     : f32,
    memory  : u64,
}



async fn metrics() -> Json<CoreSystem> {
    let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap().as_secs();

    let (cpu, used_mem, total_mem) = system::get_metrics();

    Json(CoreSystem {
    core: CoreMetrics  {
        cpu             : cpu,
        memory_used     : used_mem / 1024 / 1024,
        memory_total    : total_mem / 1024 / 1024,
        timestamp       : timestamp,
    },
    process: Some(vec![]),
    })
}

pub fn create_routes() -> Router {
    Router::new().route("/api/metrics", get(metrics))
}