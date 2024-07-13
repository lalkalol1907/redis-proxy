use std::error::Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct RedisCredentials {
    pub name: String,
    pub host: String,
    pub username: String,
    pub password: String,
}

pub struct Config {
    pub port: String,
    pub redis_credentials: Vec<RedisCredentials>,
}

impl Config {
    pub fn build() -> Self {
        Config::parse().unwrap_or_else(|e| {
            panic!("Error loading config")
        })
    }

    fn parse() -> Result<Self, Box<dyn Error>> {
        // let port = env::var("port")?;
        let port = "11000".to_owned();

        // let unparsed_redis_credentials = env::var("redis_credentials")?;
        let unparsed_redis_credentials = r#"[{"name": "redis1","host": "localhost:8000","username": "","password": ""}]"#;

        let redis_credentials: Vec<RedisCredentials> = serde_json::from_str(&*unparsed_redis_credentials)?;

        Ok(Self {
            port,
            redis_credentials,
        })
    }
}