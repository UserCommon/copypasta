use serde::{Deserialize, Serialize};
pub trait Urls {
    fn get_db_url(&self) -> String;
    fn get_url(&self) -> String;
}

/// Our config structure that deserializes settings.json
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub hostname: String,
    pub port: u16,
    pub postgres: PostgresConfig,
}

impl Config {
    /// Create Config struct with params
    pub fn new<S, U>(
        hostname: S,
        port: U,
        postgres_user: S,
        postgres_password: S,
        postgres_port: U,
        postgres_db: S,
    ) -> Self
    where
        S: Into<String>,
        U: Into<u16>,
    {
        Self {
            hostname: hostname.into(),
            port: port.into(),
            postgres: PostgresConfig::new(
                postgres_user,
                postgres_password,
                postgres_port,
                postgres_db,
            ),
        }
    }
}

impl Urls for Config {
    fn get_db_url(&self) -> String {
        format!(
            "postgres://{user}:{password}@{host}:{port}/{db}",
            user = self.postgres.user,
            password = self.postgres.password,
            host = self.hostname,
            port = self.postgres.port,
            db = self.postgres.db
        )
    }
    fn get_url(&self) -> String {
        format!("{}:{}", self.hostname, self.port)
    }
}

impl Default for Config {
    /// Non-params creation of Config in 0.0.0.0:3000
    fn default() -> Self {
        Self {
            hostname: "0.0.0.0".to_string(),
            port: 3000,
            postgres: PostgresConfig::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostgresConfig {
    pub user: String,
    pub password: String,
    pub port: u16,
    pub db: String,
}

impl PostgresConfig {
    pub fn new<S, U>(user: S, password: S, port: U, db: S) -> Self
    where
        S: Into<String>,
        U: Into<u16>,
    {
        Self {
            user: user.into(),
            password: password.into(),
            port: port.into(),
            db: db.into(),
        }
    }
}

impl Default for PostgresConfig {
    fn default() -> Self {
        Self {
            user: "postgres".to_string(),
            password: "".to_string(),

            port: 5432,
            db: "app".into(),
        }
    }
}
