use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::fs;

const BASE_PATH: &'static str = "./data";
const VERSION_FILE: &'static str = "ClientVersion.json";

#[derive(Debug)]
pub struct ConfData {
    pub versions: HashMap<String,String>, // 版本信息
}

impl ConfData {
    pub fn new() ->  Result<ConfData,Box<dyn std::error::Error>>  {
        let ver = ConfData::read_json::<HashMap<String,String>>(VERSION_FILE.to_string())?; // load versions

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

    // 获取客户端资源版本
    pub fn get_version(&self,name:String) -> String {
        match self.versions.get(&name) {
            None => "".to_string(),
            Some(str) => str.to_string(),
        }
    }
}