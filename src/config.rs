use serde::{Deserialize, Serialize};


/// Our config structure that deserializes settings.json
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub hostname: String,
    pub port: u16,
    pub postgres: PostgresConfig
}

impl Config {
    /// Create Config struct with params
    pub fn new<S, U>(
        hostname: S,
        port: U,
        postgres_user: S,
        posrgres_password: S,
        postgres_port: U
    ) -> Self
    where S: Into<String>, U: Into<u16>
    {
        Self {
            hostname: hostname.into(),
            port: port.into(),
            postgres: PostgresConfig::new(
                postgres_user,
                posrgres_password,
                postgres_port
            )
        }
    }
}

impl Default for Config {
    /// Non-params creation of Config in 0.0.0.0:3000 
    fn default() -> Self {
        Self {
            hostname: "0.0.0.0".to_string(),
            port: 3000,
            postgres: PostgresConfig::default()
        }
    }
}

impl ToString for Config {
    /// get host:port view of struct
    fn to_string(&self) -> String {
        format!("{}:{}", self.hostname, self.port)
    }
}

#[derive(Serialize, Deserialize, Debug)]

pub struct PostgresConfig {
    pub postgres_user: String,
    pub postgres_password: String,
    pub postgres_port: u16
}

impl PostgresConfig {
    pub fn new<S, U>(
        postgres_user: S,
        postgres_password: S,
        postgres_port: U
    ) -> Self
    where S: Into<String>, U: Into<u16>
    {
        Self {
            postgres_user: postgres_user.into(),
            postgres_password: postgres_password.into(),
            postgres_port: postgres_port.into()
        }
    }
}

impl Default for PostgresConfig {
    fn default() -> Self {
        Self {
            postgres_user: "postgres".to_string(),
            postgres_password: "".to_string(),
            postgres_port: 5432
        }
    }
}