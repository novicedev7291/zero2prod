use std::net::TcpListener;

use once_cell::sync::Lazy;
use sqlx::{Connection, PgConnection, PgPool, Pool, Postgres};
use uuid::Uuid;
use zero2prod::configuration::Configuration;
use zero2prod::{configuration, startup, telemetry};

static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_name = "info".into();
    let module_name = "test".into();

    if std::env::var("TEST_LOG").is_ok() {
        let subscriber =
            telemetry::trace_subscriber(module_name, default_filter_name, std::io::stdout);
        telemetry::init_tracing(subscriber);
    } else {
        let subscriber =
            telemetry::trace_subscriber(module_name, default_filter_name, std::io::sink);
        telemetry::init_tracing(subscriber);
    }
});

struct TestApp {
    address: String,
    db_pool: Pool<Postgres>,
}

async fn configure_db(config: &Configuration) -> PgPool {
    println!("Database name : {}", &config.db_config.db);
    let mut connection = PgConnection::connect(&config.db_connect_str_without_db())
        .await
        .expect("Failed to create db connection");

    sqlx::query(format!(r#"create database "{}";"#, config.db_config.db).as_str())
        .execute(&mut connection)
        .await
        .expect("Failed to create database");

    let db_pool = PgPool::connect(&config.db_connect_str())
        .await
        .expect("Failed to create DB connection pool");
    sqlx::migrate!("./migrations")
        .run(&db_pool)
        .await
        .expect("Failed to run migrations on create database");

    db_pool
}

async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind 127.0.0.1:0");
    let port = listener.local_addr().unwrap().port();

    Lazy::force(&TRACING);

    let mut configuration =
        configuration::configurations().expect("Failed to read the configurations");
    configuration.db_config.db = Uuid::new_v4().to_string();

    let db_pool = configure_db(&configuration).await;

    let server = startup::run(listener, db_pool.clone()).expect("Failed to create server");
    let _ = tokio::spawn(server);

    let address = format!("http://127.0.0.1:{}", port);

    TestApp { address, db_pool }
}

#[tokio::test]
async fn should_pass_health_check() {
    let app = spawn_app().await;

    let response = reqwest::Client::new()
        .get(format!("{}/health", &app.address))
        .send()
        .await
        .expect("failed to make a request");

    assert!(response.status().is_success());
}

#[tokio::test]
async fn subscribe_must_give_200_response() {
    let app = spawn_app().await;

    let body = "name=kuldeepxyx&email=kuldeep@xyz.com";

    let response = reqwest::Client::new()
        .post(format!("{}/subscribe", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to make subscribe request");

    assert_eq!(200, response.status().as_u16());

    let record = sqlx::query!("select email, name from subscriptions")
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to query the table");

    assert_eq!("kuldeep@xyz.com", &record.email);
    assert_eq!("kuldeepxyx", &record.name);
}

#[tokio::test]
async fn subscribe_must_give_400_response() {
    let app = spawn_app().await;

    let test_cases = vec![
        ("email=kuldeep@xyz.com", "Missing name parameter"),
        ("name=Kuldeep", "Missing email parameter"),
        ("", "Invalid request name & email both are required"),
    ];

    for (body, error_msg) in test_cases {
        let response = reqwest::Client::new()
            .post(format!("{}/subscribe", &app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to make subscribe request");

        assert_eq!(400, response.status().as_u16(), "{}", error_msg);
    }
}
