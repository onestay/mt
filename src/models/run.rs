use std::time::Duration;

use super::runner::Runner;

pub struct Run {
    id: u32,
    game_info: GameInfo,
    run_info: RunInfo,
    runners: Vec<Runner>
}

pub struct GameInfo {
    game_name: String,
    release_year: Option<i32>
}

pub struct RunInfo {
    estimate: Duration,
    category: String,
    platform: Option<String>,
}

