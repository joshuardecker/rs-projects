pub enum Error {
    OpenFileError(String),
    ReadFileError(String),
}

impl Error {
    pub fn unwrap(self) -> String {     
        match self {
            Error::OpenFileError(error) => error,
            Error::ReadFileError(error) => error,
        }
    }
}