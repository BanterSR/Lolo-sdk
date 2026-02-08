use std::fs;
use std::net::SocketAddr;
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

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
    pub async fn listener(&self) -> Result<TcpListener,Box<dyn std::error::Error>> {
        let addr = self.addr();
        tracing::info!("sdk监听地址: http://{}",addr);
        let listener = TcpListener::bind(addr).await?;
        Ok(listener)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpConfig {
    pub(crate) server: ServerInfo,
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