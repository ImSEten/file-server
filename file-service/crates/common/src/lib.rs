pub mod client;
pub mod command;
pub mod file;
pub mod server;
use thiserror::Error;

//todo Error define
#[derive(Error, Debug)]
pub enum Error {
    #[error("Error is other: {0}")]
    Other(String),
    #[error("Error is unknown")]
    Unknown,
}

//todo Result define
pub type Result<T> = std::result::Result<T, Error>;
