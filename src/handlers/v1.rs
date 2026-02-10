use axum::{extract::{Form}, Router, routing::{get, post}};
use serde::{Deserialize, Serialize};
use base64::{engine::general_purpose::STANDARD_NO_PAD, Engine as _};

use crate::{
    LoloSdkRef,
    util::aes_ecb_128_decode,
};

pub fn routes() -> Router<LoloSdkRef>{
    Router::new()
        .route("/v1/system/init",post(system_init))
        .route("/v1/user/login_by_name",post(login_by_name))
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
        Ok(data) => {
            tracing::debug!("{:?}",data)
        },
        Err(err) => {
            tracing::debug!("SystemInitRequest 解析失败{}",err)
        },
    }; // 司马完了
    "{\"result\":true,\"data\":{\"payTypes\":[{\"payTypeId\":\"226\",\"sort\":\"0\",\"backupGid\":\"0\",\"payName\":\"\\u5fae\\u4fe1\\u652f\\u4ed8\",\"rebate\":{\"rate\":1,\"rateval\":\"\",\"rateConfig\":[]}},{\"payTypeId\":\"1\",\"sort\":\"1\",\"backupGid\":\"0\",\"payName\":\"\\u652f\\u4ed8\\u5b9d\\u5feb\\u6377\",\"rebate\":{\"rate\":1,\"rateval\":\"\",\"rateConfig\":[]}}],\"version\":{\"versionName\":\"empty\",\"versionNo\":0,\"versionUrl\":\"empty\",\"updateTime\":\"empty\",\"isMust\":\"empty\",\"updateTips\":\"empty\"},\"realNameNode\":\"2\",\"productConfig\":{\"useServiceCenter\":\"2\",\"logo\":\"\",\"useSms\":\"1\",\"useBBS\":\"\",\"gift\":\"\",\"isShowFloat\":\"0\",\"autoOpenAgreement\":\"1\",\"mainLoginType\":\"3\",\"ucentUrl\":\"http:\\/\\/sdkapi-of.inutan.com\\/userCenter\\/play\",\"useCpLogin\":\"0\",\"floatLogo\":\"\",\"fcmTips\":{\"noAdultLogoutTip\":\"\\u6839\\u636e\\u6cd5\\u89c4\\u7ba1\\u63a7\\uff0c\\u5f53\\u524d\\u4e3a\\u9632\\u6c89\\u8ff7\\u7ba1\\u63a7\\u65f6\\u95f4\\uff0c\\u60a8\\u5c06\\u88ab\\u5f3a\\u5236\\u4e0b\\u7ebf\\u3002\",\"guestLoginTip\":\"\\u6839\\u636e\\u56fd\\u5bb6\\u65b0\\u95fb\\u51fa\\u7248\\u7f72\\u4e0b\\u53d1\\u300a\\u5173\\u4e8e\\u8fdb\\u4e00\\u6b65\\u4e25\\u683c\\u7ba1\\u7406 \\u5207\\u5b9e\\u9632\\u6b62\\u672a\\u6210\\u5e74\\u4eba\\u6c89\\u8ff7\\u7f51\\u7edc\\u6e38\\u620f\\u7684\\u901a\\u77e5\\u300b\\uff0c\\u4e25\\u683c\\u9650\\u5236\\u5411\\u672a\\u6210\\u5e74\\u4eba\\u63d0\\u4f9b\\u7f51\\u7edc\\u6e38\\u620f\\u670d\\u52a1\\u7684\\u65f6\\u95f4\\uff0c\\u6240\\u6709\\u7f51\\u7edc\\u6e38\\u620f\\u4f01\\u4e1a\\u4ec5\\u53ef\\u5728\\u5468\\u4e94\\u3001\\u5468\\u516d\\u3001\\u5468\\u65e5\\u548c\\u6cd5\\u5b9a\\u8282\\u5047\\u65e5\\u6bcf\\u65e520\\u65f6\\u81f321\\u65f6\\u5411\\u672a\\u6210\\u5e74\\u4eba\\u63d0\\u4f9b1\\u5c0f\\u65f6\\u670d\\u52a1\\uff0c\\u5176\\u4ed6\\u65f6\\u95f4\\u5747\\u4e0d\\u5f97\\u4ee5\\u4efb\\u4f55\\u5f62\\u5f0f\\u5411\\u672a\\u6210\\u5e74\\u4eba\\u63d0\\u4f9b\\u7f51\\u7edc\\u6e38\\u620f\\u670d\\u52a1\\u3002\",\"minorLoginTip\":\"\\u6839\\u636e\\u56fd\\u5bb6\\u65b0\\u95fb\\u51fa\\u7248\\u7f72\\u4e0b\\u53d1\\u300a\\u5173\\u4e8e\\u8fdb\\u4e00\\u6b65\\u4e25\\u683c\\u7ba1\\u7406 \\u5207\\u5b9e\\u9632\\u6b62\\u672a\\u6210\\u5e74\\u4eba\\u6c89\\u8ff7\\u7f51\\u7edc\\u6e38\\u620f\\u7684\\u901a\\u77e5\\u300b\\uff0c\\u4e25\\u683c\\u9650\\u5236\\u5411\\u672a\\u6210\\u5e74\\u4eba\\u63d0\\u4f9b\\u7f51\\u7edc\\u6e38\\u620f\\u670d\\u52a1\\u7684\\u65f6\\u95f4\\uff0c\\u6240\\u6709\\u7f51\\u7edc\\u6e38\\u620f\\u4f01\\u4e1a\\u4ec5\\u53ef\\u5728\\u5468\\u4e94\\u3001\\u5468\\u516d\\u3001\\u5468\\u65e5\\u548c\\u6cd5\\u5b9a\\u8282\\u5047\\u65e5\\u6bcf\\u65e520\\u65f6\\u81f321\\u65f6\\u5411\\u672a\\u6210\\u5e74\\u4eba\\u63d0\\u4f9b1\\u5c0f\\u65f6\\u670d\\u52a1\\uff0c\\u5176\\u4ed6\\u65f6\\u95f4\\u5747\\u4e0d\\u5f97\\u4ee5\\u4efb\\u4f55\\u5f62\\u5f0f\\u5411\\u672a\\u6210\\u5e74\\u4eba\\u63d0\\u4f9b\\u7f51\\u7edc\\u6e38\\u620f\\u670d\\u52a1\\u3002\",\"guestTimeTip\":\"\",\"minorTimeTip\":\"\",\"holidayTimeTip\":\"\",\"less8PayTip\":\"\",\"ageLimitTip\":\"\",\"ageMaxLimitTip\":\"\",\"noAdultCommonTip\":\"\",\"shiMingTip8\":\"\",\"shiMingTip8_16\":\"\",\"shiMingTip16_18\":\"\"},\"theme\":\"FF7E0C\",\"useAppAuth\":\"0\",\"switchWxAppPlug\":\"1\",\"idverifyTipTit\":\"\",\"banshuSwitch\":\"0\",\"rmAccountLg\":\"0\",\"regVerifyCode\":\"1\",\"joinQQGroup\":{\"groupNum\":\"\",\"groupKey\":\"\"},\"disFastReg\":\"1\",\"noPassWallet\":\"0\",\"hideMyFunc\":{\"hideRegBtn\":1,\"custAdReport\":0,\"normalUserBindPhone\":0,\"enableEvt\":0},\"title\":\"\",\"skinStyle\":\"0\",\"serviceInfo\":\"\",\"rmGuestLg\":0},\"useEWallet\":\"0\",\"appAuthInfo\":{\"appLogo\":\"\",\"appPackage\":\"\",\"theme\":\"FF7E0C\",\"defaultAvatar\":\"\"},\"ucentUrl\":\"\",\"subUserRole\":0,\"origPwd\":0,\"clientIp\":\"0.0.0.0\"},\"error\":{\"id\":0,\"message\":\"\"}}".to_string()
}

async fn login_by_name() {

}