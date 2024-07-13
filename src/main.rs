mod config;
mod server;
mod redis_client;

use std::{io::{Read, Write}, sync::Arc};
use std::clone::Clone;
use crate::config::{Config};
use crate::server::{ServerHandle};

#[tokio::main]
async fn main() {
    let config = Config::build();
    let config_ptr = Arc::new(config);

    let server_handle = ServerHandle::build(Arc::clone(&config_ptr));

    ServerHandle::start_listen(server_handle).await
}