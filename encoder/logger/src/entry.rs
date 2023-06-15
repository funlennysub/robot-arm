use chrono::{DateTime, Local};

pub enum LogLevel {
    Info,
    Warn,
    Error,
}

pub struct Entry {
    pub level: LogLevel,
    pub message: String,
    pub time: DateTime<Local>,
}

impl Entry {
    pub fn new(message: String, level: LogLevel) -> Self {
        Self {
            level,
            message,
            time: Local::now(),
        }
    }
}
