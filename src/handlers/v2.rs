use axum::extract::State;
use axum::{Form, Json, Router};
use axum::routing::post;
use base64::{engine::general_purpose::STANDARD,engine::general_purpose::STANDARD_NO_PAD, Engine as _};
use rand::distr::Alphanumeric;
use rand::{rng, Rng};
use rbs::{value};
use serde::{Deserialize, Serialize};
use crate::handlers::{quick, token};
use crate::{db, LoloSdkRef};
use crate::util::{aes_ecb_128_decode, des_ecb_decode};

pub fn routes() -> Router<LoloSdkRef>{
    Router::new()
        .route("/v2/users/checkLogin",post(check_login))
}

#[derive(Deserialize)]
struct AutoReq {
    data: Option<String>,
    json_data: Option<String>,
    // sign: String,
    // #[serde(rename = "productCode")]
    // product_code: Option<String>,
}

async fn auto_crypto<T:Serialize + for<'de> Deserialize<'de>>(auto: AutoReq) -> Result<T, Box<dyn std::error::Error>> {
    let (data, aes) = auto.data
        .as_ref()
        .map(|d| (d.clone(), true))
        .or_else(|| auto.json_data.as_ref().map(|json| (json.clone(), false)))
        .unwrap_or_else(|| (String::new(), true));

    let req_plaintext = if aes {
        let cleaned: String = data.chars().filter(|c| !c.is_whitespace()).collect();
        let req_ciphertext = STANDARD_NO_PAD.decode(&cleaned)?;
        let sing_key = b"0b2a18e45d7df321";
        aes_ecb_128_decode(sing_key, &req_ciphertext)?
    }else {
        let req_ciphertext = STANDARD.decode(&data)?;
        let des_key = b"78143304";
        des_ecb_decode(des_key, &req_ciphertext)?
    };

    // println!("{}", String::from_utf8_lossy(&req_plaintext));

    let json_body: T = serde_json::from_slice(&req_plaintext)?;
    Ok(json_body)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CheckLoginRequest {
    #[serde(rename = "channel_code")]
    pub channel_code: i32,
    #[serde(rename = "platform")]
    pub platform: i32,
    #[serde(rename = "device_id")]
    pub device_id: i32,
    #[serde(rename = "device_os")]
    pub device_os: i32,
    #[serde(rename = "device_os_v")]
    pub device_os_v: i32,
    #[serde(rename = "device_name")]
    pub device_name: i32,
    #[serde(rename = "imei")]
    pub imei: i32,
    #[serde(rename = "mid")]
    pub mid: i32,
    #[serde(rename = "gameName")]
    pub game_name: String,
    #[serde(rename = "sdk_v")]
    pub sdk_v: String,
    #[serde(rename = "sdk_sub_v")]
    pub sdk_sub_v: String,
    #[serde(rename = "channel_sdk_v")]
    pub channel_sdk_v: String,
    #[serde(rename = "product_v")]
    pub product_v: i32,
    #[serde(rename = "session_id")]
    pub session_id: String,
    #[serde(rename = "debug")]
    pub debug: i32,
    #[serde(rename = "screen_height")]
    pub screen_height: i32,
    #[serde(rename = "screen_width")]
    pub screen_width: i32,
    #[serde(rename = "dpi")]
    pub dpi: i32,
    #[serde(rename = "net_type")]
    pub net_type: i32,
    #[serde(rename = "time")]
    pub time: i64,
    #[serde(rename = "package_name")]
    pub package_name: String,
    #[serde(rename = "sign_md5")]
    pub sign_md5: String,
    #[serde(rename = "oaid")]
    pub oaid: String,
    #[serde(rename = "uid")]
    pub uid: String,
    #[serde(rename = "certificates")]
    pub certificates: String,
    #[serde(rename = "signIssuer")]
    pub sign_issuer: String,
    #[serde(rename = "user_name")]
    pub user_name: String,
    #[serde(rename = "signSubject")]
    pub sign_subject: String,
    #[serde(rename = "sign_sha1")]
    pub sign_sha1: String,
    #[serde(rename = "signMd5")]
    pub sign_md5_1: String,
    #[serde(rename = "androidid")]
    pub androidid: String,
    #[serde(rename = "token")]
    pub token: String,
    #[serde(rename = "check_time")]
    pub check_time: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CheckLoginResponse {
    #[serde(rename = "uid")]
    pub uid: String,
    #[serde(rename = "user_name")]
    pub user_name: String,
    #[serde(rename = "token")]
    pub token: String,
    #[serde(rename = "user_token")]
    pub user_token: String,
}

async fn check_login(
    State(state): State<LoloSdkRef>,
    Form(input): Form<AutoReq>
) -> Json<quick::Response<CheckLoginResponse>> {
    let mut rsp = quick::Response::new();
    let req = match auto_crypto::<CheckLoginRequest>(input).await {
        Ok(req) => {
            req
        },
        Err(err) => {
            rsp.set_error(format!("{}", err));
            return Json(rsp);
        },
    };
    let token = token::SdkToken::unmarshal(req.token.clone());
    let user = match db::UserData::select_by_id(&state.sdb, token.id).await {
        Ok(mut users) => {
            if users.is_empty() {
                rsp.set_error(String::from("账号未注册"));
                return Json(rsp);
            }
            users.remove(0)
        },
        Err(_) => {
            rsp.set_error(String::from("账号未注册"));
            return Json(rsp);
        }
    };
    if user.user_token != Some(req.token.clone()) {
        rsp.set_error(String::from("账号未注册"));
        return Json(rsp);
    }
    // 获取 gate token 信息
    let mut gate_checks = match db::GateCheck::select_by_map(&state.sdb,value!{"uid":&user.id.to_string()}).await {
        Ok(gate_checks) => {gate_checks}
        Err(_) => {
            rsp.set_error(String::from("账号未注册"));
            return Json(rsp);
        }
    };
    let mut gate_check = if gate_checks.len() < 1 {
        // 写入记录
        let gate_check = db::GateCheck{
            uid: user.id.to_string(),
            gate_token: Option::from(req.package_name),
            last_package_name: Option::from(token.marshal()),
            gen_key: None,
        };
        let _ = db::GateCheck::insert(&state.sdb,&gate_check).await;
        gate_check
    }else {
        gate_checks.remove(0)
    };
    // 刷新token
    let gen_key :String = rng()
        .sample_iter(&Alphanumeric)
        .take(16)
        .map(char::from)
        .collect();
    let token = token::GateToken::new(user.id.to_string(),gen_key.clone());
    gate_check.gate_token = Option::from(token.marshal());
    let _ = match db::GateCheck::update_by_map(&state.sdb,&gate_check,value!{"uid":&gate_check.uid}).await{
        Ok(_) => {}
        Err(err) => {
            rsp.set_error(format!("{}", err));
            return Json(rsp);
        }
    };
    rsp.set_data(CheckLoginResponse{
        uid: gate_check.uid,
        user_name: user.username,
        token: req.token,
        user_token: gate_check.gate_token.unwrap(),
    });

    Json(rsp)
}