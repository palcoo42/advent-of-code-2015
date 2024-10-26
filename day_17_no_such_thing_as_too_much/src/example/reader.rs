use std::path::Path;

use common::reader::{text_reader::TextReader, text_reader_error::TextReaderError};

use super::eggnog::Eggnog;

pub struct Reader {}

impl Reader {
    pub fn read_eggnog(path: &Path) -> Result<Eggnog, TextReaderError> {
        let reader = TextReader::new(path.to_path_buf());
        let lines = reader.read_lines(20)?;

        let mut buckets = Vec::new();

        for line in lines {
            let bucket = line.parse::<u32>().map_err(|err| {
                TextReaderError::GenericError(format!(
                    "Failed to parse line '{}' to u32 with error '{}'",
                    line, err
                ))
            })?;

            buckets.push(bucket);
        }

        Ok(Eggnog::new(buckets))
    }
}
