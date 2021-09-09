use chrono;
use sqlx;
use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    id: i32,
    created_at: chrono::DateTime<chrono::Utc>,
    admin: bool,
    name: String,
    password: Vec<u8>
}