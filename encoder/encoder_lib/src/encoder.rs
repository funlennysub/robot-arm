use std::{fmt, fs::OpenOptions, path::Path};

use serialport::SerialPort;

use crate::{command::Command, Result};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Step {
    pub command: Command,
    pub times: u8,
}

impl fmt::Display for Step {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{times}{cmd}", times = self.times, cmd = self.command)
    }
}

#[derive(Debug, Default)]
pub struct Encoder {
    pub port: Option<Box<dyn SerialPort>>,
    pub steps: Vec<Step>,
}

impl Encoder {
    pub fn connect(&mut self, port: String, baud_rate: u32) -> Result<()> {
        let port = serialport::new(port, baud_rate).open()?;
        self.port = Some(port);

        Ok(())
    }

    pub fn list_ports() -> Result<Vec<String>> {
        Ok(serialport::available_ports()
            .map(|p| p.iter().map(|p| p.port_name.clone()).collect())?)
    }

    pub fn clear_steps(&mut self) {
        self.steps.clear();
    }

    pub fn add_step(&mut self, command: Command) -> Result<()> {
        command.run(&mut self.port)?;

        if let Some(step) = self.steps.last_mut() {
            if step.command == command {
                step.times += 1;
                return Ok(());
            }
        }

        self.steps.push(Step { command, times: 1 });
        Ok(())
    }

    pub fn run_all(&mut self) -> Result<()> {
        if let Some(port) = self.port.as_mut() {
            let steps = &self.steps;
            let binding = steps
                .iter()
                .map(Step::to_string)
                .collect::<Vec<_>>()
                .join("");
            let bytes = binding.as_bytes();
            port.write_all(bytes)?;
        }

        Ok(())
    }

    pub fn step_count(&self) -> u32 {
        self.steps.iter().map(|s| s.times as u32).sum()
    }

    pub fn localize(&self) -> String {
        let mut localized = String::with_capacity(self.step_count() as usize);

        for step in &self.steps {
            localized += &step.to_string();
        }

        localized
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        let file = OpenOptions::new()
            .append(false)
            .create(true)
            .write(true)
            .open(path)?;
        serde_json::to_writer(file, &self.steps)?;

        Ok(())
    }

    pub fn load(&mut self, path: &Path) -> Result<()> {
        let file = OpenOptions::new().read(true).open(path)?;
        self.steps = serde_json::from_reader(file)?;

        Ok(())
    }
}
