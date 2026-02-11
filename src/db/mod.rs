use serde::{Deserialize, Serialize};
use rbatis::{impl_select, RBatis};
use crate::config;

pub async fn new(db_type: config::DbType,url: &str) -> Result<RBatis,Box<dyn std::error::Error>> {
    let rb = RBatis::new();
    match db_type {
        config::DbType::Sqlite=>{
            rb.link(rbdc_sqlite::driver::SqliteDriver {}, url).await?;
        },
        config::DbType::Mysql=>{
            rb.link(rbdc_mysql::driver::MysqlDriver{}, url).await?;
        },
    }
    tracing::info!("初始化数据库完成");
    Ok(rb)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserData {
    pub id: i64,
    pub username:String,
    pub password:String,
    pub reg_device:Option<String>,
    pub user_token:Option<String>,
    pub auth_token:Option<String>,
}
rbatis::crud!(UserData {},"of_quick");
impl_select!(UserData{select_by_id(id:i64) => "`where id = #{id} limit 1`"},"of_quick");
impl_select!(UserData{select_by_name(username:String) => "`where username = #{username} limit 1`"},"of_quick");


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GateCheck {
    pub uid:String,
    pub gate_token:Option<String>,
    pub last_package_name:Option<String>,
    pub gen_key:Option<String>,
}
rbatis::crud!(GateCheck {},"of_quick_check");