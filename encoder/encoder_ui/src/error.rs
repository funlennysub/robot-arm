use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum Error {
    #[error(transparent)]
    Lib(#[from] encoder_lib::error::Error),
    #[error(transparent)]
    Eframe(#[from] eframe::Error),
    #[error(transparent)]
    Io(#[from] io::Error),
}
