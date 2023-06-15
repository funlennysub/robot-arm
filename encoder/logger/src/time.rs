use chrono::{DateTime, Local};

pub trait DateFormat {
    fn date_short(&self) -> String;
}

impl DateFormat for DateTime<Local> {
    fn date_short(&self) -> String {
        self.format("%H:%M:%S%.3f").to_string()
    }
}
