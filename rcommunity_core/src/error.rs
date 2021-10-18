use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Not yet implemented.")]
    NotImplemented,
    #[error("Unknown error: {0}.")]
    UnknownError(String),
}