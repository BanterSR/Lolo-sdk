use rbatis::RBatis;
use crate::db::user::UserData;

pub async fn init_db() -> RBatis {
    let rb = RBatis::new();

    // sqlite
    rb.link(rbdc_sqlite::driver::SqliteDriver {}, "sqlite://target/sqlite.db").await.unwrap();

    let data = UserData::insert(
        &rb, &UserData{uid: Option::from("114514".to_string()) },
    ).await;
    rb
}