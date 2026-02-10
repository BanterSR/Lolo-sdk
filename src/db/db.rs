use rbatis::RBatis;

pub async fn init_db() -> RBatis {
    let rb = RBatis::new();
    // sqlite
    rb.link(rbdc_sqlite::driver::SqliteDriver {}, "sqlite://target/sqlite.db").await.unwrap();
    rb
}