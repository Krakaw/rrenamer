use thiserror::Error;
use std::num::ParseIntError;

#[derive(Error, Debug)]
pub enum RrenamerError {
    #[error("Movie Not Found")]
    MovieNotFound,
    #[error("Invalid filename")]
    InvalidFilename,
    #[error(transparent)]
    ApiError(#[from] reqwest::Error),
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),
}
