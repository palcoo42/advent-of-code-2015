use std::{error::Error, fmt::Display, io};

#[derive(Debug)]
pub enum ReaderError {
    OpenFileError(String, io::Error),
}

impl Error for ReaderError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ReaderError::OpenFileError(_path, error) => Some(error),
        }
    }
}

impl Display for ReaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReaderError::OpenFileError(path, error) => {
                write!(f, "Failed to open file '{}', error: {}", path, error)
            }
        }
    }
}

impl From<io::Error> for ReaderError {
    fn from(value: io::Error) -> Self {
        ReaderError::OpenFileError(String::new(), value)
    }
}
