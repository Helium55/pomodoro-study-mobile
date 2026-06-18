pub mod data;
pub mod goals;
pub mod notify;
pub mod pomodoros;
pub mod settings;
pub mod stats;
pub mod tasks;

pub fn now_ms() -> i64 {
    chrono::Utc::now().timestamp_millis()
}

pub fn today_local() -> String {
    chrono::Local::now().format("%Y-%m-%d").to_string()
}
