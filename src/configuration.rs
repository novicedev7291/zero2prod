#[derive(serde::Deserialize)]
pub struct Configuration {
    port: u16,
    pub db_config: DBConfiguration,
}

#[derive(serde::Deserialize)]
pub struct DBConfiguration {
    port: u16,
    user: String,
    password: String,
    pub db: String,
    host: String,
}

impl Configuration {
    pub fn address(&self) -> String {
        format!("127.0.0.1:{}", self.port)
    }

    pub fn db_connect_str(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.db_config.user,
            self.db_config.password,
            self.db_config.host,
            self.db_config.port,
            self.db_config.db
        )
    }

    pub fn db_connect_str_without_db(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}",
            self.db_config.user, self.db_config.password, self.db_config.host, self.db_config.port
        )
    }
}

pub fn configurations() -> Result<Configuration, config::ConfigError> {
    let settings = config::Config::builder()
        .add_source(config::File::with_name("configuration.yml"))
        .build()
        .expect("Failed to read configurations!!!");

    settings.try_deserialize::<Configuration>()
}
