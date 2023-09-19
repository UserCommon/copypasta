use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub hostname: String,
    pub port: u32
}

impl Config {
    pub fn new<T, U>(hostname: String, port: u32) -> Self
    where T: Into<String>, U: Into<u32>
    {
        Self {
            hostname: hostname.into(),
            port: port.into()
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            hostname: "0.0.0.0".to_string(),
            port: 3000
        }
    }
}

impl ToString for Config {
    fn to_string(&self) -> String {
        format!("{}:{}", self.hostname, self.port)
    }
}