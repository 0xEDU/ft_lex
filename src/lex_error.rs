use std::fmt;

#[derive(Debug)]
pub enum LexError {
    FrontendError(String),
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexError::FrontendError(msg) => write!(f, "Frontend error: {}", msg),
        }
    }
}
