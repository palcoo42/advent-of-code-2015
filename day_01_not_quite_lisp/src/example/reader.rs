use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use super::{building::Building, reader_error::ReaderError};

pub struct Reader {
    path: PathBuf,
}

impl Reader {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn read(&self) -> Result<Building, ReaderError> {
        let file = File::open(self.path.clone()).map_err(|err| {
            ReaderError::OpenFileError(self.path.to_str().unwrap().to_string(), err)
        })?;
        let mut reader = BufReader::new(file);

        let mut line = String::with_capacity(1_000_000);
        let _size = reader.read_line(&mut line)?;

        Ok(Building::new(&line))
    }
}
