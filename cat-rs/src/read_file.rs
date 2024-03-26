use std::{fs::File, io::Read};
use crate::error::Error;

pub fn read_file_to_string(file_path: &String) -> Result<String, Error> {
    let file_result = File::open(file_path);

    let mut file = match file_result {
        Ok(file) => file,
        Err(error) => return Err(Error::OpenFileError(
            error.to_string())
        ),
    };

    let mut buffer = String::new();

    if let Err(error) = file.read_to_string(&mut buffer) {
        return Err(Error::ReadFileError(
            error.to_string())
        )
    }

    Ok(buffer)
}