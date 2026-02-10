use crate::LoloSdkRef;

use axum::{Router,routing::{get,post},extract::Request,Json};
use http::StatusCode;
use serde::{Serialize};
use serde_json::{json, Value};
use chrono::Utc;

pub fn routes() -> Router<LoloSdkRef>{
    Router::new()
        .route("/resolve",get(routing))
        .route("/config",get(te_config))
        .route("/sync",post(te_config))
}

#[derive(Serialize)]
struct ResolveInfo {
    #[serde(rename = "accountType")]
    host:String,
    ttl:i32,
    ips:Vec<String>,
    cip:String,
    cl:Vec<i32>,
}

async fn routing(req: Request)->(StatusCode,Json<ResolveInfo>) {
    (StatusCode::OK,Json(ResolveInfo{
        host:req.uri().query().unwrap_or("domain").to_string(),
        ttl: 60,
        ips: Vec::new(),
        cip: String::new(),
        cl: Vec::new(),
    }))
}

async fn te_config()-> (StatusCode,Json<Value>) {
    (StatusCode::OK, Json(json!({
        "code": 0,
        "data": {
            "server_timestamp": Utc::now().timestamp_millis(),
            "sync_batch_size": 100,
            "sync_interval": 90,
        },
        "msg": ""
    })))
}