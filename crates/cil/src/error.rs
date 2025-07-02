use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Loaded PE binary is not a valid CIL image")]
    InvalidCilImage,

    #[error("Failed to read CIL image: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Invalid CIL image: {0}")]
    ParseError(#[from] binrw::Error),

    #[error("Unsupported CIL table {0}")]
    UnsupportedTable(&'static str),
}

pub type Result<T> = std::result::Result<T, Error>;
