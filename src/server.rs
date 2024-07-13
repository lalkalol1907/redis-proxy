use std::io::Error;
use std::str;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;

use crate::config::Config;
use crate::redis_client::redis_client::RedisClient;

pub struct ServerHandle {
    server: Server
}

impl ServerHandle {
    pub fn build(config: Arc<Config>) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Self { server: Server {config} }))
    }

    pub async fn start_listen(this: Arc<Mutex<Self>>) {
        let port = this.lock().await.server.config.port.clone();
        let listener = TcpListener::bind("127.0.0.1:".to_owned() + &*port).await.unwrap();

        loop {
            let (stream, addr) = listener.accept().await.unwrap();
            println!("Accepted connection from {addr}");
            let this_cloned = this.clone();
            tokio::spawn(async move {
                match this_cloned.lock().await.server.handle_request(stream).await {
                    Ok(_) => println!("Success"),
                    Err(e) => println!("Error: {e}"),
                }
                println!("Disconnected from {addr}");
            });
        }
    }
}

struct Server {
    config: Arc<Config>,
}

impl Server {
    pub fn build(config: Arc<Config>) -> Self {
        return Self {
            config,
        };
    }

    pub async fn handle_request(&self, mut stream: TcpStream) -> Result<(), Error> {
        let mut buf = [0; 1024];
        while stream.read(&mut buf).await.unwrap() != 0 {
            let request = str::from_utf8(&buf).unwrap();
            let response = self.process_request(request).unwrap();

            stream.write_all(response).await.unwrap();
        }
        return Ok(());
    }

    pub fn process_request(&self, query: &str) -> Result<&[u8], Error> {
        let redis_client = RedisClient::build(self.config.clone());

        let split: Vec<_> = query.split("\r\n").collect();
        let command = split[2].to_lowercase();
        let args: Vec<_> = split[3..split.len() - 1]
            .into_iter()
            .enumerate()
            .filter(|&(i, _)| i % 2 != 0)
            .map(|(_, e)| *e)
            .collect();

        println!("{}", command);
        println!("{:?}", args);

        return match command.as_str() {
            "ping" => {
                Ok(b"+PONG\r\n")
            }
            _ => {
                Ok(b"+PONG\r\n")
            }
        };
    }
}