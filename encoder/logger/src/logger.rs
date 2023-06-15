use egui::Widget;
use egui_extras::{TableBuilder, Column};

use crate::entry::{Entry, LogLevel};

#[derive(Default)]
pub struct Logger {
    entries: Vec<Entry>,
}

impl Logger {
    pub fn info(&mut self, message: String) {
        self.entries.push(Entry::new(message, LogLevel::Info))
    }

    pub fn warn(&mut self, message: String) {
        self.entries.push(Entry::new(message, LogLevel::Warn))
    }

    pub fn error(&mut self, message: String) {
        self.entries.push(Entry::new(message, LogLevel::Error))
    }
}

impl Widget for Logger {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        TableBuilder::new(ui)
            .striped(true)
            .columns(Column::auto().resizable(false), 3)
            .header(30., |mut header| {

            })
            .body(|mut body| {

            })
    }
}