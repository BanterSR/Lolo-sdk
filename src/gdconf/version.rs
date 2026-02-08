use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug,Serialize,Deserialize)]
pub struct ClientVersion {
    #[serde(rename = "VersionMap")]
    pub version_map: HashMap<String,String>
}

impl ClientVersion {
    // 获取客户端资源版本
    pub fn get_version(&self,name:String) -> String {
        match self.version_map.get(&name) {
            None => "".to_string(),
            Some(str) => str.to_string(),
        }
    }
}