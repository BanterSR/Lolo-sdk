use rbatis::RBatis;
use crate::config;

pub struct SDKDB {
    rb: RBatis,
}

impl SDKDB {
    pub async fn new(db_type: config::DbType,url: &str) -> Result<Self,Box<dyn std::error::Error>> {
        let rb = RBatis::new();
        match db_type {
            config::DbType::Sqlite=>{
                rb.link(rbdc_sqlite::driver::SqliteDriver {}, url).await?;
            },
            config::DbType::Mysql=>{
                rb.link(rbdc_mysql::driver::MysqlDriver{}, url).await?;
            },
        }
        Ok(SDKDB{
            rb,
        })
    }
}