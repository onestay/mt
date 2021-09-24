use thiserror::Error;
use anyhow::Result;
use sqlx::{Pool, Any};
use log::{info};
use tide::{Server};

pub struct MTConfig {
    pub port: u16
}

pub struct MTApp {
    server: Server<ServerState>,
    config: MTConfig
}

#[derive(Debug, Clone)]
struct ServerState {
    db: Pool<Any>
}

pub fn new(config: MTConfig, db_pool: Pool<Any>) -> Result<MTApp> {
    let server_state = ServerState{db: db_pool};
    let app = tide::with_state(server_state);
    Ok(MTApp{config, server: app})
}

impl MTApp {
    pub async fn run(self) -> Result<()> {
        info!("Starting MarathonTools ver. {}", env!("CARGO_PKG_VERSION"));
        self.server.at("/").get((index);
        self.server.listen("127.0.0.1:8080").await?;
        Ok(())
    }
}


async fn index(req: &mut tide::Request<ServerState>) -> tide::Result {
    Ok("Hello World".into())
}

#[derive(Debug, Error)]
pub enum MTError {
    #[error("This is a test error")]
    TestError
}
