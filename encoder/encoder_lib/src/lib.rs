pub mod command;
pub mod encoder;
pub mod error;

pub type Result<T> = std::result::Result<T, error::Error>;
