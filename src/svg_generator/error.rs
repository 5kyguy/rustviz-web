use std::fmt;

#[derive(Debug)]
pub enum RustvizError {
    Io(String),
    Parse(String),
}

impl fmt::Display for RustvizError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RustvizError::Io(s) | RustvizError::Parse(s) => write!(f, "{}", s),
        }
    }
}

impl std::error::Error for RustvizError {}

impl From<std::io::Error> for RustvizError {
    fn from(e: std::io::Error) -> Self {
        RustvizError::Io(e.to_string())
    }
}
