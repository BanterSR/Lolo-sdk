use http::StatusCode;
use serde::{Deserialize, Serialize};
use crate::{LoloSdkRef};
use axum::{extract::State, routing::{get, post}, Form, Json, Router};

pub fn routes() -> Router<LoloSdkRef>{
    Router::new()
        .route("/dispatch/region_info", post(region_info))
}

#[derive(Deserialize)]
struct RegionForm {
    version:String,
    version2:String,
    #[serde(rename = "accountType")]
    account_type:String,
    os:String,
    #[serde(rename = "lastloginsdkuid")]
    lastloginsdkuid:String,
}


#[derive(Serialize)]
struct RegionInfo {
    status:bool,
    message:String,
    gate_tcp_ip:String,
    gate_tcp_port:u16,
    is_server_open:bool,
    text:String,
    client_log_tcp_ip:String,
    client_log_tcp_port:u16,
    #[serde(rename = "currentVersion")]
    current_version:String,
    photo_share_cdn_url:String,
}

impl RegionInfo {
    fn new() -> Self {
        Self{
            status:true,
            message: "success".to_string(),
            gate_tcp_ip:"127.0.0.1".to_string(),
            gate_tcp_port:20001,
            is_server_open:false,
            text:"".to_string(),
            client_log_tcp_ip:"127.0.0.1".to_string(),
            client_log_tcp_port:20002,
            current_version:"2099-12-31-23-59-59_2099-12-31-23-59-59".to_string(),
            photo_share_cdn_url:"https://cdn-photo-of.inutan.com/cn_prod_main".to_string(),
        }
    }
}

async fn region_info(
    State(state): State<LoloSdkRef>,
    Form(form): Form<RegionForm>
) -> (StatusCode, Json<RegionInfo>) {
    let mut info = RegionInfo::new();

    info.is_server_open = state.cfg.gate.open;
    info.gate_tcp_ip = state.cfg.gate.server.ip.to_string();
    info.gate_tcp_port = state.cfg.gate.server.port;

    info.client_log_tcp_ip = state.cfg.client_log.server.ip.to_string();
    info.client_log_tcp_port = state.cfg.client_log.server.port;

    info.current_version = form.version;

    (StatusCode::OK, Json(info))
}