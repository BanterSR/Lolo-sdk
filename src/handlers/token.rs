use axum::{ Json, Router};
use axum::extract::State;
use axum::routing::post;
use rand::{rng, Rng};
use rand::distr::Alphanumeric;
use rbs::value;
use serde::{Deserialize, Serialize};
use crate::{db, LoloSdkRef};

#[derive(Debug, Serialize, Deserialize)]
pub struct SdkToken{
    pub id:i64,
    pub time:i64,
    pub key:String,
}

impl SdkToken {
    pub fn new(id:i64) -> Self {
        let key = rng()
            .sample_iter(&Alphanumeric)
            .take(16)
            .map(char::from)
            .collect();
        Self{
            id,
            time: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64,
            key,
        }
    }

    pub fn marshal(&self) ->String {
        let bytes = serde_json::to_vec(&self).unwrap();
        逆天转换(&bytes)
    }

    pub fn unmarshal(str:String) -> Self{
        let token:SdkToken = serde_json::from_slice(&*逆天转回(str)).unwrap();
        token
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GateToken{
    pub id:String,
    pub time:i64,
    pub key:String,
}

impl GateToken {
    pub fn new(id:String,key:String) -> Self {
        Self{
            id,
            time: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64,
            key,
        }
    }

    pub fn marshal(&self) ->String {
        let bytes = serde_json::to_vec(&self).unwrap();
        逆天转换(&bytes)
    }

    pub fn unmarshal(str:String) -> Self{
        let token:GateToken = serde_json::from_slice(&*逆天转回(str)).unwrap();
        token
    }
}

const SEG: &str = "☃️";

pub fn 逆天转换(bin: &Vec<u8>) -> String {
    let mut str = String::new();
    for &b in bin {
        str.push_str(SEG);
        str.push_str(&(b as i32 + 100).to_string());
    }
    str
}

pub fn 逆天转回(str: String) -> Vec<u8> {
    str.split(SEG)
        .filter(|s| !s.is_empty())
        .filter_map(|s| s.parse::<i32>().ok())
        .filter(|&i| i >= 100 && i <= 355)
        .map(|i| (i - 100) as u8)
        .collect()
}

pub fn routes() -> Router<LoloSdkRef>{
    Router::new()
        .route("/gucooing/lolo/checkSdkToken",post(check_sdk_token))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CheckSdkTokenRequest {
    #[serde(rename = "token")]
    pub token: String,
    #[serde(rename = "uid")]
    pub uid: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CheckSdkTokenResponse {
    #[serde(rename = "code")]
    pub code: i32,
    #[serde(rename = "uid")]
    pub uid: String,
}

async fn check_sdk_token(
    State(state): State<LoloSdkRef>,
    Json(req): Json<CheckSdkTokenRequest>
) -> Json<CheckSdkTokenResponse> {
    let token = GateToken::unmarshal(req.token.clone());
    let mut rsp = CheckSdkTokenResponse{
        code: 0,
        uid: "".to_string(),
    };
    let gate_check = match db::GateCheck::select_by_map(&state.sdb, value!{"uid":&token.id}).await {
        Ok(mut gate_check) => {
            if gate_check.is_empty() {
                rsp.code = -1;
                return Json(rsp);
            }
            gate_check.remove(0)
        },
        Err(_) => {
            rsp.code = -1;
            return Json(rsp);
        }
    };
    if gate_check.gate_token.unwrap().to_string() != req.token {
        rsp.code = 2;
        return Json(rsp);
    };
    rsp.uid = gate_check.uid;
    Json(rsp)
}