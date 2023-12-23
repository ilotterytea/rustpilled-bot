use std::time::Instant;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref START_TIME: Instant = Instant::now();
}

pub const DEFAULT_COMMAND_DELAY_SEC: i32 = 5;
pub const DEFAULT_COMMAND_OPTIONS: Vec<String> = Vec::new();
pub const DEFAULT_COMMAND_SUBCOMMANDS: Vec<String> = Vec::new();

pub const DEFAULT_PREFIX: &str = "~";
pub const DEFAULT_LANGUAGE: &str = "english";

pub const HOLIDAY_V1_API_URL: &str = "https://hol.ilotterytea.kz/api/v1";
pub const SEVENTV_WEBSOCKET_URL: &str = "wss://events.7tv.io/v3";

pub const TIMER_CHECK_DELAY: u64 = 1;
