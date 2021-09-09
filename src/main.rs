use anyhow::Result;
use chrono;
use dotenv;
use fern::Dispatch;
use log::warn;
use mt::MTConfig;
use sqlx::{any::AnyPoolOptions, Any, Pool};
use std::env;

#[async_std::main]
async fn main() -> Result<()> {
    dotenv::dotenv()?;
    set_up_logging()?;
    let db_pool = set_up_db().await?;
    let config = new_config()?;

    let mt_app = mt::new(config, db_pool)?;
    mt_app.run().await?;

    Ok(())
}

fn set_up_logging() -> Result<(), fern::InitError> {
    Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{} [{}]: {}",
                record.level(),
                chrono::Utc::now(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .apply()?;

    Ok(())
}

async fn set_up_db() -> Result<Pool<Any>> {
    return Ok(AnyPoolOptions::new().connect("sqlite://db/test.db").await?);
}

fn new_config() -> Result<MTConfig> {
    let port = env::var("PORT")
        .unwrap_or_else(|e| {
            warn!("Can't get PORT env var, {}, using default 808", e);
            String::from("8080")
        })
        .parse::<u16>()
        .unwrap_or_else(|e| {
            warn!("Invalid PORT, {}, using default 8080", e);
            8080
        });

        Ok(MTConfig{port})
}
