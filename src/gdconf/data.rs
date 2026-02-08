use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::fs;
use tokio::select;
use crate::gdconf::version;

const BASE_PATH: &'static str = "./data";
const VERSION_FILE: &'static str = "ClientVersion.json";

#[derive(Debug)]
pub struct ConfData {
    pub versions: version::ClientVersion, // 版本信息
}

impl ConfData {
    pub fn new() ->  Result<ConfData,Box<dyn std::error::Error>>  {
        let time1 = std::time::Instant::now();
        tracing::info!("开始读取资源文件");
        let ver = ConfData::read_json::<version::ClientVersion>(VERSION_FILE.to_string())?; // load versions

        tracing::info!("读取资源文件完成,用时:{} ms", time1.elapsed().as_millis());
        Ok(ConfData {
            versions: ver,
        })
    }

    // 读取json文件
    fn read_json<T:Serialize + for<'de> Deserialize<'de>>(file:String) -> Result<T,Box<dyn std::error::Error>> {
        let path = format!("{}/{}",BASE_PATH,file);
        let json_content = fs::read_to_string(path)?;
        let json_t: T = serde_json::from_str(&json_content)?;
        tracing::info!("读取文件:{}",file);
        Ok(json_t)
    }
}