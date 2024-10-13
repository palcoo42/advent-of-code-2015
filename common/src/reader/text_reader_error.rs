use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum TextReaderError {
    FileOpenError(String, std::io::Error),
    GenericError(String),
}

impl Display for TextReaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let desc = match self {
            TextReaderError::FileOpenError(path, err) => {
                format!("Failed to open file '{}' with error '{}'", path, err)
            }
            TextReaderError::GenericError(err) => err.to_owned(),
        };
        write!(f, "{}", desc)
    }
}

impl Error for TextReaderError {}
