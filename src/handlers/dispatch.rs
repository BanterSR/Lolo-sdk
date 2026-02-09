use crate::LoloSdkRef;
use axum::{extract::State, routing::{get,post}, Form, Json, Router};
use http::StatusCode;
use serde::{Deserialize, Serialize};

pub fn routes() -> Router<LoloSdkRef>{
    Router::new()
        .route("/dispatch/region_info", post(region_info).
            head(head_region_info).get(head_region_info))
        .route("/dispatch/client_hot_update", post(client_hot_update))
        .route("/dispatch/get_login_url_list",post(|| async { "" }))
        .route("/dispatch/get_client_black_list", get(get_client_black_list))
}

#[allow(dead_code)]
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
            gate_tcp_ip:String::new(),
            gate_tcp_port:0,
            is_server_open:false,
            text:String::new(),
            client_log_tcp_ip:String::new(),
            client_log_tcp_port:0,
            current_version:String::new(),
            photo_share_cdn_url:String::new(),
        }
    }
}

async fn region_info(
    State(state): State<LoloSdkRef>,
    Form(form): Form<RegionForm>
) -> (StatusCode, Json<RegionInfo>) {
    let mut info = RegionInfo::new();

    info.is_server_open = state.cfg.gate.open;
    info.gate_tcp_ip = state.cfg.gate.server.ip.clone();
    info.gate_tcp_port = state.cfg.gate.server.port;

    info.client_log_tcp_ip = state.cfg.client_log.server.ip.clone();
    info.client_log_tcp_port = state.cfg.client_log.server.port;

    info.current_version = state.dcfg.versions.get_version(form.version);
    info.photo_share_cdn_url = state.cfg.http.photo_share_cdn_url.clone();

    (StatusCode::OK, Json(info))
}

async fn head_region_info() ->StatusCode {
    StatusCode::OK
}

#[derive(Debug, Clone, Serialize, Deserialize)]
 struct GMClientConfig {
     status: bool,
     message: String,
    #[serde(rename = "hotOssUrl")]
     hot_oss_url: String,
    #[serde(rename = "currentVersion")]
     current_version: String,
     server: String,
    #[serde(rename = "ssAppId")]
     ss_app_id: String,
    #[serde(rename = "ssServerUrl")]
     ss_server_url: String,
     open_gm: bool,
     open_error_log: bool,
    #[serde(rename = "open_netConnecting_log")]
     open_net_connecting_log: bool,
    #[serde(rename = "ipAddress")]
     ip_address: String,
    #[serde(rename = "payUrl")]
     pay_url: String,
    #[serde(rename = "isTestServer")]
     is_test_server: bool,
     error_log_level: i32,
     server_id: String,
     open_cs: bool,
}

async fn client_hot_update(
    State(state): State<LoloSdkRef>,
    Form(form): Form<RegionForm>
) -> (StatusCode, Json<GMClientConfig>) {
    (StatusCode::OK, Json(GMClientConfig{
        status: true,
        message: "success".to_string(),
        hot_oss_url: state.cfg.http.hot_oss_url.clone(),
        current_version: state.dcfg.versions.get_version(form.version),
        server: "Lolo".to_string(),
        ss_app_id: "c969ebf346794cc797ed6eb6c3eac089".to_string(),
        ss_server_url: format!("https://{}:{}",state.cfg.http.outer_ip,state.cfg.http.tls_port),
        open_gm: true,
        open_error_log: true,
        open_net_connecting_log: true,
        ip_address: String::new(),
        pay_url: "http://api-callback-of.inutan.com:19701".to_string(),
        is_test_server: true,
        error_log_level: 0,
        server_id: String::new(),
        open_cs: true,
    }))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ClientBlack {
    #[serde(rename = "ID")]
    id :i64,
    #[serde(rename = "manufacturer")]
    manufacturer:String,
    #[serde(rename = "model")]
    model:String,
}

async fn get_client_black_list() ->  (StatusCode, Json<Vec<ClientBlack>>)  {
    (StatusCode::OK, Json(vec![
        ClientBlack{id: 100, manufacturer: "RETRY_LIMITATION".to_string(), model: "4".to_string()},
        ClientBlack{id: 600, manufacturer: "HUAWEI".to_string(), model: String::new()},
		ClientBlack{id: 1000, manufacturer: "Samsung".to_string(), model: String::new()},
		ClientBlack{id: 2000, manufacturer: "Sony".to_string(), model: String::new()},
    ]))
}