use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum EncoderError {}

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
    #[error(transparent)]
    SerialPort(#[from] serialport::Error),
    #[error(transparent)]
    Io(#[from] io::Error),
}
