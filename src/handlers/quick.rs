use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Response<T> {
    #[serde(rename = "result")]
    pub result: bool,
    #[serde(rename = "status")]
    pub status: bool,
    #[serde(rename = "data")]
    pub data: Option<T>,
    #[serde(rename = "message")]
    pub message: String,
    #[serde(rename = "error")]
    pub error: Option<Error>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "message")]
    pub message: String,
}

impl<T> Response<T> {
    pub fn new() -> Self {
        Self{
            result:true,
            status:true,
            data: None,
            message: String::new(),
            error: None,
        }
    }

    pub fn set_data(&mut self, data: T) {
        self.data = Some(data);
    }

    pub fn set_error(&mut self, message: String) {
        self.error = Option::from(Error {
            id:1,
            message,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginByNameRequest {
    #[serde(rename = "authToken")]
    pub auth_token: String,
    #[serde(rename = "clientLang")]
    pub client_lang: String,
    #[serde(rename = "deviceId")]
    pub device_id: String,
    #[serde(rename = "platform")]
    pub platform: i32,
    #[serde(rename = "password")]
    pub password: String,
    #[serde(rename = "productCode")]
    pub product_code: String,
    #[serde(rename = "andId")]
    pub and_id: String,
    #[serde(rename = "gameVersion")]
    pub game_version: i64,
    #[serde(rename = "signMd5")]
    pub sign_md5: String,
    #[serde(rename = "imei")]
    pub imei: String,
    #[serde(rename = "sdkVersion")]
    pub sdk_version: i64,
    #[serde(rename = "time")]
    pub time: i64,
    #[serde(rename = "oaid")]
    pub oaid: String,
    #[serde(rename = "isEmt")]
    pub is_emt: String,
    #[serde(rename = "username")]
    pub username: String,
    #[serde(rename = "channelCode")]
    pub chanel_code: String,
}