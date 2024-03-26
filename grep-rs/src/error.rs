#[derive(Debug)]
pub enum Error {
    ReadFileError(String),
    FileToStringError(String),
}