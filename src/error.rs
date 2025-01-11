use serde::Serialize;

#[derive(Debug, thiserror::Error, Serialize)]
pub enum Error {
    #[error("{0}")]
    Io(String),
    #[error("{0}")]
    Tauri(String)
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err.to_string())
    }
}

impl From<tauri::Error> for Error {
    fn from(err: tauri::Error) -> Self {
        Error::Tauri(err.to_string())
    }
}

pub type Result<T> = std::result::Result<T, Error>;
