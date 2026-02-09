use std::fs;
use std::net::SocketAddr;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub(crate) http: HttpConfig,
    pub(crate) gate: GateConfig,
    pub(crate) client_log:ClientLog,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerInfo {
    pub(crate) ip: String,
    pub(crate) port: u16,
}

impl ServerInfo {
    pub fn addr(&self) -> SocketAddr {
        SocketAddr::new(self.ip.parse().unwrap(), self.port)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpConfig {
    pub(crate) server: ServerInfo,
    pub(crate) outer_ip:String,
    pub(crate) tls_port:u16,
    pub(crate) data_path:String,
    pub(crate) photo_share_cdn_url: String,
    pub(crate) hot_oss_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct  GateConfig {
    pub(crate) server: ServerInfo,
    pub(crate) open: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientLog {
    pub(crate) server: ServerInfo,
}

pub fn read_config() -> Result<Config, Box<dyn std::error::Error>> {
    let json_content = fs::read_to_string("./conf/config.json")?;
    let config: Config = serde_json::from_str(&json_content)?;
    Ok(config)
}