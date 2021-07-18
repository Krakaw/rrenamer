use thiserror::Error;

#[derive(Error, Debug)]
pub enum RrenamerError {
    #[error("Movie Not Found")]
    MovieNotFound,
    #[error("Invalid filename: {0}")]
    InvalidFilename(String),
    #[error("Invalid file extension: {0}")]
    InvalidFileExt(String),
    #[error("Invalid path: {0}")]
    InvalidPath(String),

    #[error(transparent)]
    ApiError(#[from] reqwest::Error),
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}
