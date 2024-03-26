#[derive(Debug)]
pub enum Error {
    ReadDirectoryError(String),
    ConvertFileNameError(String),
    ConvertOSStringError,
    GetFileTypeError(String),
}