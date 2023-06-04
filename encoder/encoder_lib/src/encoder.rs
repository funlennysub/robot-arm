use crate::{command::Command, Result};
use serialport::{COMPort, SerialPortInfo};
use std::{fs::OpenOptions, io::Write, path::Path};

#[derive(Debug, Default)]
pub struct Encoder {
    pub port: Option<COMPort>,
    pub commands: Vec<Command>,
}

impl Encoder {
    pub fn connect(&mut self, port: String, baud_rate: u32) -> Result<()> {
        let port = serialport::new(port, baud_rate).open_native()?;
        self.port = Some(port);

        Ok(())
    }

    pub fn list_ports() -> Result<Vec<SerialPortInfo>> {
        Ok(serialport::available_ports()?)
    }

    pub fn clear_commands(&mut self) {
        self.commands.clear();
    }

    pub fn run_all(&mut self) -> Result<()> {
        if let Some(port) = self.port.as_mut() {
            let command = &self.commands;
            let binding = command
                .iter()
                .map(Command::to_string)
                .collect::<Vec<_>>()
                .join("");
            let bytes = binding.as_bytes();
            port.write_all(bytes)?;
        }

        Ok(())
    }

    pub fn command_count(&self) -> u32 {
        self.commands.iter().map(|c| c.times() as u32).sum()
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        let file = OpenOptions::new()
            .append(false)
            .create(true)
            .write(true)
            .open(path)?;
        serde_json::to_writer(file, &self.commands)?;

        Ok(())
    }

    pub fn load(&mut self, path: &Path) -> Result<()> {
        let file = OpenOptions::new().read(true).open(path)?;
        self.commands = serde_json::from_reader(file)?;

        Ok(())
    }
}
