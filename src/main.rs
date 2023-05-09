use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::{configuration, startup, telemetry};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber =
        telemetry::trace_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    telemetry::init_tracing(subscriber);

    let config = configuration::configurations().expect("Failed to load configurations");
    let listener =
        TcpListener::bind(config.address()).expect("Failed to bind to address specified in config");

    let db_pool = PgPool::connect(&config.db_connect_str())
        .await
        .expect("Failed to create db connection pool");
    startup::run(listener, db_pool)?.await
}
