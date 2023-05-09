use env_logger::Env;
use std::net::TcpListener;

use sqlx::PgPool;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let config = zero2prod::configurations().expect("Failed to load configurations");
    let listener =
        TcpListener::bind(config.address()).expect("Failed to bind to address specified in config");

    let db_pool = PgPool::connect(&config.db_connect_str())
        .await
        .expect("Failed to create db connection pool");
    zero2prod::run(listener, db_pool)?.await
}
