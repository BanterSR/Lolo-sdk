mod router;
mod config;
mod handlers;
mod gdconf;
mod db;
mod util;

use handlers::{
    sub,
    dispatch,
};

use gdconf::{
    data,
};

use std::{process::exit,path::PathBuf,sync::OnceLock};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use axum_server::tls_rustls::RustlsConfig;
use rbatis::RBatis;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{filter::Targets, fmt};

struct LoloSdk {
    cfg:config::Config,
    dcfg:data::ConfData,
    sdb:RBatis,
}
type LoloSdkRef = &'static LoloSdk;

impl LoloSdk {
    pub async fn new() ->  Result<Self, Box<dyn std::error::Error>> {
        // 初始化配置文件
        let cfg = config::read_config()?;
        // 初始化log
        let filter = Targets::new()
            .with_target("Lolo_sdk", tracing::Level::DEBUG)
            .with_default(tracing::Level::INFO);

        tracing_subscriber::registry()
            .with(fmt::layer().with_target(false))
            .with(filter)
            .init();
        tracing::info!("初始化tracing完成");
        // 初始化data
        let dcfg = data::ConfData::new()?;
        // 初始化数据库
        let sdb = db::new(
            cfg.sdk.db_type.to_owned(),
            &cfg.sdk.db_url.clone(),
        ).await?;

        Ok(Self{
            cfg,
            dcfg,
            sdb,
        })
    }

    async fn listener(&self) -> Result<(TcpListener, RustlsConfig),Box<dyn std::error::Error>> {
        let addr = self.cfg.sdk.server.addr();
        tracing::info!("sdk监听地址: http://{}",addr);
        let listener = TcpListener::bind(addr).await?;

        let config = RustlsConfig::from_pem_file(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("data")
                .join("cert.pem"),
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("data")
                .join("key.pem"),
        ).await?;

        Ok((listener,config))
    }

    pub fn tls_addr(&self) -> SocketAddr {
       let addr = SocketAddr::new(self.cfg.sdk.server.ip.parse().unwrap(), self.cfg.sdk.tls_port);
        tracing::info!("sdk监听地址: https://{}",addr);
        addr
    }
}

#[tokio::main]
async fn main() {
    use rustls::crypto::ring::default_provider;
    default_provider()
        .install_default()
        .expect("无法设置默认 CryptoProvider");
    let sdk = match LoloSdk::new().await {
        Ok(sdk) => {
            tracing::info!("初始化sdk完成");
            sdk
        },
        Err(err) => {
            eprintln!("初始化sdk失败 err:{}", err);
            exit(1);
        },
    };
    static STATE: OnceLock<LoloSdk> = OnceLock::new();
    let _ = STATE.set(sdk);

    let listeners = match  STATE.get().unwrap().listener().await {
        Ok(listeners) => {
            tracing::info!("初始化http服务器完成");
            listeners
        },
        Err(err) => {
            tracing::error!("初始化http服务器失败 err:{}",err);
            return
        }
    };
    let app = router::router(STATE.get().unwrap());

    tracing::info!("Lolo Sdk 启动!");
    let app_clone = app.clone();
    let http_server = tokio::spawn(async move {
        axum::serve(listeners.0, app_clone).await.expect("sdk炸了");
    });
    let https_server = tokio::spawn(async move {
        axum_server::bind_rustls(
            STATE.get().unwrap().tls_addr(),
            listeners.1).serve(app.into_make_service())
            .await.expect("sdk炸了");
    });
    tokio::select! {
        _ = http_server => {},
        _ = https_server => {},
        _ = tokio::signal::ctrl_c() => {
            println!("Lolo 关闭!");
        }
    }
}