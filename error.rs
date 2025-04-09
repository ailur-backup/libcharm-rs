#[derive(Debug)]
pub struct Error {
    pub message: String,
}

impl Error {
    pub fn new(message: &str) -> Self {
        Error {
            message: message.to_string(),
        }
    }

    pub fn from_string(message: String) -> Self {
        Error { message }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for Error {}