use thiserror::Error;

pub type TracerResult<T> = std::result::Result<T, TracerError>;

#[derive(Error, Debug)]
pub enum TracerError {
    #[error("fs error")]
    FsError(#[from] std::io::Error),
}