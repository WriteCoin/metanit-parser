use std::fmt;

#[derive(Debug)]
pub enum ErrorKind {
    Http,
    Parse,
    Encoding,
    InvalidUrl,
    MenuParse,
    CodeBlockParse,
    Cache,
    Timeout,
}

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl Error {
    pub fn new(kind: ErrorKind, message: impl Into<String>) -> Self {
        Error {
            kind,
            message: message.into(),
            source: None,
        }
    }

    pub fn with_source(
        kind: ErrorKind,
        message: impl Into<String>,
        source: Box<dyn std::error::Error + Send + Sync>,
    ) -> Self {
        Error {
            kind,
            message: message.into(),
            source: Some(source),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}: {}", self.kind, self.message)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source
            .as_ref()
            .map(|s| s.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::with_source(ErrorKind::Http, "HTTP request failed", Box::new(e))
    }
}

impl From<url::ParseError> for Error {
    fn from(e: url::ParseError) -> Self {
        Error::with_source(ErrorKind::InvalidUrl, "Invalid URL", Box::new(e))
    }
}

pub type Result<T> = std::result::Result<T, Error>;
