use serde::Serialize;

#[derive(Debug, thiserror::Error, Serialize)]
pub enum Error {
    #[error("Tauri error: {0}")]
    Tauri(String),
    #[error("IO error: {0}")]
    Io(String),
    #[error("Windows error: {0}")]
    Windows(String),
    #[error("{0}")]
    Message(String),
}

impl From<tauri::Error> for Error {
    fn from(error: tauri::Error) -> Self {
        Error::Tauri(error.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::Io(error.to_string())
    }
}

impl From<windows::core::Error> for Error {
    fn from(error: windows::core::Error) -> Self {
        Error::Windows(error.to_string())
    }
}

pub type Result<T> = std::result::Result<T, Error>;
