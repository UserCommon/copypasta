use serde::{Deserialize, Serialize};


/// Our config structure that deserializes settings.json
#[derive(Serialize, Deserialize)]
pub struct Config {
    pub hostname: String,
    pub port: u32
}

impl Config {
    /// Create Config struct with params
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
    /// Non-params creation of Config in 0.0.0.0:3000 
    fn default() -> Self {
        Self {
            hostname: "0.0.0.0".to_string(),
            port: 3000
        }
    }
}

impl ToString for Config {
    /// get host:port view of struct
    fn to_string(&self) -> String {
        format!("{}:{}", self.hostname, self.port)
    }
}