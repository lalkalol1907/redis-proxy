use crate::config::{Config, RedisCredentials};
use std::sync::Arc;
use redis::Connection;

extern crate redis;

pub struct RedisClient {
    connections: Vec<Arc<Connection>>
}

impl RedisClient {
    pub fn build(config: Arc<Config>) -> RedisClient {
        let redis_credentials = (*config).redis_credentials.clone();

        let connections = redis_credentials
            .iter()
            .map(|rc| Self::create_connection(rc)).collect();

        return Self { connections };
    }

    fn create_connection(credentials: &RedisCredentials) -> Arc<Connection> {
        let url = credentials.host.clone();
        let client = redis::Client::open("redis_client://".to_owned() + &*url).unwrap();
        let con = client.get_connection().unwrap();
        Arc::new(con)
    }
}