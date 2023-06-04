use crate::Result;
use serialport::COMPort;
use std::{fmt, io::Write};

/// Commands that the interpreter understands
#[derive(Clone, Copy, serde::Serialize, serde::Deserialize)]
#[serde(tag = "action", content = "times")]
pub enum Command {
    /// Rotate base to the left
    Left(u8),
    /// Rotate base to the right
    Right(u8),
    /// Return robot to initial position
    IPos(u8),
    /// Move shoulder motor down
    ShoulderDowm(u8),
    /// Move shoulder motor up
    ShoulderUp(u8),
    /// Move elbow motor down
    ElbowDown(u8),
    /// Move elbow motor up
    ElbowUp(u8),
    /// Move gripper up
    GripperDown(u8),
    /// Move gripper down
    GripperUp(u8),
    /// Close the gripper
    GripperClose(u8),
    /// Open the gripper
    GripperOpen(u8),
}

impl Command {
    pub fn run(&self, port: &mut Option<COMPort>) -> Result<()> {
        if let Some(port) = port {
            port.write_all(format!("{self:?}").as_bytes())?;
        }
        Ok(())
    }
}

impl Command {
    pub fn times(&self) -> u8 {
        match self {
            Command::Left(t) => *t,
            Command::Right(t) => *t,
            Command::IPos(t) => *t,
            Command::ShoulderDowm(t) => *t,
            Command::ShoulderUp(t) => *t,
            Command::ElbowDown(t) => *t,
            Command::ElbowUp(t) => *t,
            Command::GripperDown(t) => *t,
            Command::GripperUp(t) => *t,
            Command::GripperClose(t) => *t,
            Command::GripperOpen(t) => *t,
        }
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Command::Left(_) => write!(f, "L"),
            Command::Right(_) => write!(f, "R"),
            Command::IPos(_) => write!(f, "I"),
            Command::ShoulderDowm(_) => write!(f, "V"),
            Command::ShoulderUp(_) => write!(f, "G"),
            Command::ElbowDown(_) => write!(f, "B"),
            Command::ElbowUp(_) => write!(f, "H"),
            Command::GripperDown(_) => write!(f, "N"),
            Command::GripperUp(_) => write!(f, "J"),
            Command::GripperClose(_) => write!(f, "C"),
            Command::GripperOpen(_) => write!(f, "O"),
        }
    }
}

impl fmt::Debug for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::Left(t) => write!(f, "{t}L"),
            Command::Right(t) => write!(f, "{t}R"),
            Command::IPos(t) => write!(f, "{t}I"),
            Command::ShoulderDowm(t) => write!(f, "{t}V"),
            Command::ShoulderUp(t) => write!(f, "{t}G"),
            Command::ElbowDown(t) => write!(f, "{t}B"),
            Command::ElbowUp(t) => write!(f, "{t}H"),
            Command::GripperDown(t) => write!(f, "{t}N"),
            Command::GripperUp(t) => write!(f, "{t}J"),
            Command::GripperClose(t) => write!(f, "{t}C"),
            Command::GripperOpen(t) => write!(f, "{t}O"),
        }
    }
}
