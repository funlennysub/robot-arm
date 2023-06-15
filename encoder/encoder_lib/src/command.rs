use encoder_proc_macro::Command;
use serialport::SerialPort;
use std::fmt;

use crate::Result;

/// Commands that the interpreter understands
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, Command)]
pub enum Command {
    /// Rotate base to the left
    #[command("L")]
    Left,
    /// Rotate base to the right
    #[command("R")]
    Right,
    /// Return robot to initial position
    #[command("I")]
    IPos,
    /// Move shoulder motor down
    #[command("V")]
    ShoulderDown,
    /// Move shoulder motor up
    #[command("G")]
    ShoulderUp,
    /// Move elbow motor down
    #[command("B")]
    ElbowDown,
    /// Move elbow motor up
    #[command("H")]
    ElbowUp,
    /// Move gripper up
    #[command("N")]
    GripperDown,
    /// Move gripper down
    #[command("J")]
    GripperUp,
    /// Close the gripper
    #[command("C")]
    GripperClose,
    /// Open the gripper
    #[command("O")]
    GripperOpen,
}

impl Command {
    pub fn run(&self, port: &mut Option<Box<dyn SerialPort>>) -> Result<()> {
        if let Some(port) = port {
            port.write_all(self.to_string().as_bytes())?;
        }

        Ok(())
    }
}
