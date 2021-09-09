use chrono;

use super::run::Run;
pub struct Marathon {
    id: u32,
    created_at: chrono::DateTime<chrono::Utc>,
    runs: Vec<Run>,
}
