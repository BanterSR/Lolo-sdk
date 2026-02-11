use std::{
    time::{SystemTime, UNIX_EPOCH},
    collections::HashMap,
};
use axum::{extract::{Form, State}, Router, routing::{post}, Json};
use serde::{Deserialize, Serialize};
use base64::{engine::general_purpose::STANDARD_NO_PAD, Engine as _};
use rbs::{value};
use crate::{LoloSdkRef, util::aes_ecb_128_decode, handlers::quick, db};
use crate::handlers::token;

pub fn routes() -> Router<LoloSdkRef>{
    Router::new()
        .route("/v1/system/init",post(system_init))
        .route("/v1/user/loginByName",post(login_by_name))
        .route("/v1/auth/asyUonline",post(asy_uonline))
        .route("/v1/auth/getUserInfo",post(get_user_info))
}

#[derive(Deserialize)]
struct AutoReq {
    data: String,
    // sign: String,
    // #[serde(rename = "productCode")]
    // product_code: Option<String>,
}

async fn auto_crypto<T:Serialize + for<'de> Deserialize<'de>>(auto: AutoReq) -> Result<T, Box<dyn std::error::Error>> {
    // Base64 解码
    let cleaned: String = auto.data.chars().filter(|c| !c.is_whitespace()).collect();
    let req_ciphertext = STANDARD_NO_PAD.decode(&cleaned)?;

    // AES 解密
    let sing_key = b"0b2a18e45d7df321";
    let req_plaintext = aes_ecb_128_decode(sing_key, &req_ciphertext)?;

    // println!("{}", String::from_utf8_lossy(&req_plaintext));

    let json_body: T = serde_json::from_slice(&req_plaintext)?;
    Ok(json_body)
}

#[derive(Debug, Deserialize, Serialize)]
struct SystemInitRequest {
    #[serde(rename = "screenWidth")]
    screen_width: String,
    #[serde(rename = "latitude")]
    latitude: String,
    #[serde(rename = "authToken")]
    auth_token: String,
    #[serde(rename = "deviceName")]
    device_name: String,
    #[serde(rename = "deviceId")]
    device_id: String,
    #[serde(rename = "platform")]
    platform: i32,
    #[serde(rename = "osVersion")]
    os_version: String,
    #[serde(rename = "countryCode")]
    country_code: String,
    #[serde(rename = "andId")]
    and_id: String,
    #[serde(rename = "dpi")]
    dpi: String,
    #[serde(rename = "osLanguage")]
    os_language: String,
    #[serde(rename = "oaid")]
    oaid: String,
    #[serde(rename = "longitude")]
    longitude: String,
    #[serde(rename = "channelCode")]
    channel_code: String,
    #[serde(rename = "netType")]
    net_type: String,
    #[serde(rename = "screenHeight")]
    screen_height: String,
    #[serde(rename = "clientLang")]
    client_lang: String,
    #[serde(rename = "ismobiledevice")]
    is_mobile_device: String,
    #[serde(rename = "flashversion")]
    flash_version: String,
    #[serde(rename = "osName")]
    os_name: String,
    #[serde(rename = "pushToken")]
    push_token: String,
    #[serde(rename = "productCode")]
    product_code: String,
    #[serde(rename = "isjailbroken")]
    is_jailbroken: String,
    #[serde(rename = "javasupport")]
    java_support: String,
    #[serde(rename = "imei")]
    imei: String,
    #[serde(rename = "gameVersion")]
    game_version: i32,
    #[serde(rename = "signMd5")]
    sign_md5: String,
    #[serde(rename = "sdkVersion")]
    sdk_version: i32,
    #[serde(rename = "time")]
    time: i64,
    #[serde(rename = "defaultbrowser")]
    default_browser: String,
    #[serde(rename = "isEmt")]
    is_emt: String,
}

async fn system_init(Form(input): Form<AutoReq>) ->String {
    let _req = match auto_crypto::<SystemInitRequest>(input).await {
        Ok(_req) => {
            _req
        },
        Err(err) => {
            return err.to_string();
        },
    }; // 司马完了
    "{\"result\":true,\"data\":{\"payTypes\":[{\"payTypeId\":\"226\",\"sort\":\"0\",\"backupGid\":\"0\",\"payName\":\"\\u5fae\\u4fe1\\u652f\\u4ed8\",\"rebate\":{\"rate\":1,\"rateval\":\"\",\"rateConfig\":[]}},{\"payTypeId\":\"1\",\"sort\":\"1\",\"backupGid\":\"0\",\"payName\":\"\\u652f\\u4ed8\\u5b9d\\u5feb\\u6377\",\"rebate\":{\"rate\":1,\"rateval\":\"\",\"rateConfig\":[]}}],\"version\":{\"versionName\":\"empty\",\"versionNo\":0,\"versionUrl\":\"empty\",\"updateTime\":\"empty\",\"isMust\":\"empty\",\"updateTips\":\"empty\"},\"realNameNode\":\"2\",\"productConfig\":{\"useServiceCenter\":\"2\",\"logo\":\"\",\"useSms\":\"1\",\"useBBS\":\"\",\"gift\":\"\",\"isShowFloat\":\"0\",\"autoOpenAgreement\":\"1\",\"mainLoginType\":\"3\",\"ucentUrl\":\"http:\\/\\/sdkapi-of.inutan.com\\/userCenter\\/play\",\"useCpLogin\":\"0\",\"floatLogo\":\"\",\"fcmTips\":{\"noAdultLogoutTip\":\"\\u6839\\u636e\\u6cd5\\u89c4\\u7ba1\\u63a7\\uff0c\\u5f53\\u524d\\u4e3a\\u9632\\u6c89\\u8ff7\\u7ba1\\u63a7\\u65f6\\u95f4\\uff0c\\u60a8\\u5c06\\u88ab\\u5f3a\\u5236\\u4e0b\\u7ebf\\u3002\",\"guestLoginTip\":\"\\u6839\\u636e\\u56fd\\u5bb6\\u65b0\\u95fb\\u51fa\\u7248\\u7f72\\u4e0b\\u53d1\\u300a\\u5173\\u4e8e\\u8fdb\\u4e00\\u6b65\\u4e25\\u683c\\u7ba1\\u7406 \\u5207\\u5b9e\\u9632\\u6b62\\u672a\\u6210\\u5e74\\u4eba\\u6c89\\u8ff7\\u7f51\\u7edc\\u6e38\\u620f\\u7684\\u901a\\u77e5\\u300b\\uff0c\\u4e25\\u683c\\u9650\\u5236\\u5411\\u672a\\u6210\\u5e74\\u4eba\\u63d0\\u4f9b\\u7f51\\u7edc\\u6e38\\u620f\\u670d\\u52a1\\u7684\\u65f6\\u95f4\\uff0c\\u6240\\u6709\\u7f51\\u7edc\\u6e38\\u620f\\u4f01\\u4e1a\\u4ec5\\u53ef\\u5728\\u5468\\u4e94\\u3001\\u5468\\u516d\\u3001\\u5468\\u65e5\\u548c\\u6cd5\\u5b9a\\u8282\\u5047\\u65e5\\u6bcf\\u65e520\\u65f6\\u81f321\\u65f6\\u5411\\u672a\\u6210\\u5e74\\u4eba\\u63d0\\u4f9b1\\u5c0f\\u65f6\\u670d\\u52a1\\uff0c\\u5176\\u4ed6\\u65f6\\u95f4\\u5747\\u4e0d\\u5f97\\u4ee5\\u4efb\\u4f55\\u5f62\\u5f0f\\u5411\\u672a\\u6210\\u5e74\\u4eba\\u63d0\\u4f9b\\u7f51\\u7edc\\u6e38\\u620f\\u670d\\u52a1\\u3002\",\"minorLoginTip\":\"\\u6839\\u636e\\u56fd\\u5bb6\\u65b0\\u95fb\\u51fa\\u7248\\u7f72\\u4e0b\\u53d1\\u300a\\u5173\\u4e8e\\u8fdb\\u4e00\\u6b65\\u4e25\\u683c\\u7ba1\\u7406 \\u5207\\u5b9e\\u9632\\u6b62\\u672a\\u6210\\u5e74\\u4eba\\u6c89\\u8ff7\\u7f51\\u7edc\\u6e38\\u620f\\u7684\\u901a\\u77e5\\u300b\\uff0c\\u4e25\\u683c\\u9650\\u5236\\u5411\\u672a\\u6210\\u5e74\\u4eba\\u63d0\\u4f9b\\u7f51\\u7edc\\u6e38\\u620f\\u670d\\u52a1\\u7684\\u65f6\\u95f4\\uff0c\\u6240\\u6709\\u7f51\\u7edc\\u6e38\\u620f\\u4f01\\u4e1a\\u4ec5\\u53ef\\u5728\\u5468\\u4e94\\u3001\\u5468\\u516d\\u3001\\u5468\\u65e5\\u548c\\u6cd5\\u5b9a\\u8282\\u5047\\u65e5\\u6bcf\\u65e520\\u65f6\\u81f321\\u65f6\\u5411\\u672a\\u6210\\u5e74\\u4eba\\u63d0\\u4f9b1\\u5c0f\\u65f6\\u670d\\u52a1\\uff0c\\u5176\\u4ed6\\u65f6\\u95f4\\u5747\\u4e0d\\u5f97\\u4ee5\\u4efb\\u4f55\\u5f62\\u5f0f\\u5411\\u672a\\u6210\\u5e74\\u4eba\\u63d0\\u4f9b\\u7f51\\u7edc\\u6e38\\u620f\\u670d\\u52a1\\u3002\",\"guestTimeTip\":\"\",\"minorTimeTip\":\"\",\"holidayTimeTip\":\"\",\"less8PayTip\":\"\",\"ageLimitTip\":\"\",\"ageMaxLimitTip\":\"\",\"noAdultCommonTip\":\"\",\"shiMingTip8\":\"\",\"shiMingTip8_16\":\"\",\"shiMingTip16_18\":\"\"},\"theme\":\"FF7E0C\",\"useAppAuth\":\"0\",\"switchWxAppPlug\":\"1\",\"idverifyTipTit\":\"\",\"banshuSwitch\":\"0\",\"rmAccountLg\":\"0\",\"regVerifyCode\":\"1\",\"joinQQGroup\":{\"groupNum\":\"\",\"groupKey\":\"\"},\"disFastReg\":\"1\",\"noPassWallet\":\"0\",\"hideMyFunc\":{\"hideRegBtn\":1,\"custAdReport\":0,\"normalUserBindPhone\":0,\"enableEvt\":0},\"title\":\"\",\"skinStyle\":\"0\",\"serviceInfo\":\"\",\"rmGuestLg\":0},\"useEWallet\":\"0\",\"appAuthInfo\":{\"appLogo\":\"\",\"appPackage\":\"\",\"theme\":\"FF7E0C\",\"defaultAvatar\":\"\"},\"ucentUrl\":\"\",\"subUserRole\":0,\"origPwd\":0,\"clientIp\":\"0.0.0.0\"},\"error\":{\"id\":0,\"message\":\"\"}}".to_string()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResultV1 {
    #[serde(rename = "authToken")]
    pub auth_token: String,
    #[serde(rename = "userData")]
    pub user_data: Option<UserDataV1>,
    #[serde(rename = "checkRealName")]
    pub check_real_name: i32,
    #[serde(rename = "isAdult")]
    pub is_adult: bool,
    #[serde(rename = "uAge")]
    pub u_age: i32,
    #[serde(rename = "ckPlayTime")]
    pub ck_play_time: i32,
    #[serde(rename = "guestRealName")]
    pub guest_real_name: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "message")]
    pub message: String,
    #[serde(rename = "extInfo")]
    pub ext_info: Option<ExtInfo>,
}

impl LoginResultV1 {
    fn new(user: db::UserData) ->Self {
        Self{
            auth_token: user.auth_token.clone().expect("REASON"),
            user_data: Option::from(UserDataV1::new(user)),
            check_real_name: 0,
            is_adult: true,
            u_age: 9999,
            ck_play_time: 0,
            guest_real_name: 1,
            id: 0,
            message: String::new(),
            ext_info: Option::from(ExtInfo{
                oauth_type: 0,
                    oauth_id: String::new(),
                access_token: String::new(),
            }),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserDataV1 {
    #[serde(rename = "uid")]
    pub uid: String,
    #[serde(rename = "username")]
    pub username: String,
    #[serde(rename = "token")]
    pub token: String,
    #[serde(rename = "isGuest")]
    pub is_guest: i32,
    #[serde(rename = "isMbUser")]
    pub is_mb_user: i32,
    #[serde(rename = "isSnsUser")]
    pub is_sns_user: i32,
    #[serde(rename = "mobile")]
    pub mobile: String,
}

impl UserDataV1 {
    fn new(user: db::UserData) ->Self {
        Self{
            uid: user.id.to_string(),
            username: user.username.to_string(),
            token: user.user_token.clone().unwrap_or(String::new()),
            is_guest: 0,
            is_mb_user: 1,
            is_sns_user: 0,
            mobile: "188****8888".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtInfo {
    #[serde(rename = "oauthType")]
    pub oauth_type: i32,
    #[serde(rename = "oauthId")]
    pub oauth_id: String,
    #[serde(rename = "access_token")]
    pub access_token: String,
}

async fn login_by_name(
    State(state): State<LoloSdkRef>,
    Form(input): Form<AutoReq>
) -> Json<quick::Response<LoginResultV1>> {
    let mut rsp = quick::Response::new();
    let req = match auto_crypto::<quick::LoginByNameRequest>(input).await {
        Ok(req) => {
            req
        },
        Err(err) => {
            rsp.set_error(format!("{}", err));
            return Json(rsp);
        },
    };
    let mut user = match db::UserData::select_by_name(&state.sdb, req.username).await {
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
    // 刷新auth token 和 user token
    let auth_token = token::SdkToken::new(user.id);
    let user_token = token::SdkToken::new(user.id);
    user.auth_token = Some(auth_token.marshal());
    user.user_token = Some(user_token.marshal());
    let _ = match db::UserData::update_by_map(&state.sdb,&user,value!{"id":&user.id}).await{
       Ok(_) => {}
       Err(err) => {
           rsp.set_error(format!("{}", err));
           return Json(rsp);
       }
    };

    rsp.set_data(LoginResultV1::new(user));

    Json(rsp)
}

async fn asy_uonline() -> Json<quick::Response<LoginResultV1>> {
    let rsp = quick::Response::new();
    Json(rsp)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserExtraInfoRequest {
    #[serde(rename = "authToken")]
    pub auth_token: String,
    #[serde(rename = "clientLang")]
    pub client_lang: String,
    #[serde(rename = "deviceId")]
    pub device_id: String,
    #[serde(rename = "platform")]
    pub platform: i32,
    #[serde(rename = "uid")]
    pub uid: String,
    #[serde(rename = "productCode")]
    pub product_code: String,
    #[serde(rename = "andId")]
    pub and_id: String,
    #[serde(rename = "gameVersion")]
    pub game_version: i32,
    #[serde(rename = "signMd5")]
    pub sign_md5: String,
    #[serde(rename = "imei")]
    pub imei: String,
    #[serde(rename = "sdkVersion")]
    pub sdk_version: i32,
    #[serde(rename = "time")]
    pub time: i64,
    #[serde(rename = "oaid")]
    pub oaid: String,
    #[serde(rename = "isEmt")]
    pub is_emt: String,
    #[serde(rename = "channelCode")]
    pub channel_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[repr(i32)]
pub enum SexType {
    #[serde(rename = "0")]
    Undefined = 0,
    #[serde(rename = "1")]
    Male = 1,
    #[serde(rename = "2")]
    Female = 2,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserExtraInfo {
    #[serde(rename = "isBindPhone")]
    pub is_bind_phone: i32,
    #[serde(rename = "nickName")]
    pub nick_name: String,
    #[serde(rename = "phone")]
    pub phone: String,
    #[serde(rename = "sexType")]
    pub sex_type: SexType,
    #[serde(rename = "regType")]
    pub reg_type: String,
    #[serde(rename = "lastLoginTime")]
    pub last_login_time: String,
    #[serde(rename = "fcmShowTips")]
    pub fcm_show_tips: i32,
    #[serde(rename = "isAdult")]
    pub is_adult: i32,
    #[serde(rename = "timeleft")]
    pub time_left: i32,
    #[serde(rename = "bindInfo")]
    pub bind_info: HashMap<String,BindQd>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BindQd {
    #[serde(rename = "isBind")]
    pub is_bind: i32,
    #[serde(rename = "bid")]
    pub bid: i32,
    #[serde(rename = "buid")]
    pub buid: String,
}

async fn get_user_info(
    State(state): State<LoloSdkRef>,
    Form(input): Form<AutoReq>
) -> Json<quick::Response<UserExtraInfo>> {
    let mut rsp = quick::Response::new();
    let req = match auto_crypto::<UserExtraInfoRequest>(input).await {
        Ok(req) => {
            req
        },
        Err(err) => {
            rsp.set_error(format!("{}", err));
            return Json(rsp);
        },
    };
    let token = token::SdkToken::unmarshal(req.auth_token.clone());
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
    if user.auth_token != Some(req.auth_token) {
        rsp.set_error(String::from("账号未注册"));
        return Json(rsp);
    }

    let mut data = UserExtraInfo{
        is_bind_phone: 1,
        nick_name: user.username,
        phone: "188****8888".to_string(),
        sex_type: SexType::Undefined,
        reg_type: "3".to_string(),
        last_login_time: SystemTime::now().duration_since(UNIX_EPOCH).
            expect("REASON").as_secs().to_string(),
        fcm_show_tips: 0,
        is_adult: 1,
        time_left: 0,
        bind_info: HashMap::new(),
    };
    data.bind_info.insert("bindWX".to_string(), BindQd{ is_bind: 0, bid: 4, buid: "".to_string(), });
    data.bind_info.insert("bindQQ".to_string(), BindQd{ is_bind: 0, bid: 5, buid: "".to_string(), });
    data.bind_info.insert("bindApple".to_string(), BindQd{ is_bind: 0, bid: 16, buid: "".to_string(), });

    rsp.set_data(data);

    Json(rsp)
}