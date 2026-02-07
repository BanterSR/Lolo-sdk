mod router;
mod config;
mod handlers;
mod gdconf;

use handlers::{
    dispatch,
};

use gdconf::{
    data,
};

use std::process::exit;
use std::sync::OnceLock;

#[derive(Debug)]
struct LoloSdk {
    cfg:config::Config,
    dcfg:data::ConfData,
}
type LoloSdkRef = &'static LoloSdk;

impl LoloSdk {
    pub async fn new() ->  Result<Self, Box<dyn std::error::Error>> {
        // 初始化配置文件
        let cfg = config::read_config()?;
        // 初始化log
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::TRACE)
            .with_level(true)
            .init();
        tracing::info!("初始化tracing完成");
        // 初始化data
        let dcfg = data::ConfData::new()?;
        // 初始化数据库

        Ok(Self{
            cfg,
            dcfg,
        })
    }
}

#[tokio::main]
async fn main() {
    match LoloSdk::new().await{
        Ok(sdk )=> {
            static STATE: OnceLock<LoloSdk> = OnceLock::new();
            STATE.set(sdk).expect("TODO: panic message");
            let listener = match  STATE.get().unwrap().cfg.http.server.listener().await {
                Ok(listener) => {
                    tracing::info!("初始化TcpListener完成");
                    listener
                },
                Err(err) => {
                    tracing::error!("初始化http服务器失败 err:{}",err);
                    return
                }
            };
            let app = router::router(STATE.get().unwrap());

            axum::serve(listener, app).await.expect("use std::process::exit;");
        }
        Err(err) => {
            eprintln!("初始化LoloSdk失败 err:{}", err);
            exit(1);
        }
    }
}