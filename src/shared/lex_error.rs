use std::{fmt, io};

#[derive(Debug)]
pub enum LexError {
    GenericError(String),
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexError::GenericError(msg) => write!(f, "{}", msg),
        }
    }
}

impl From<io::Error> for LexError {
    fn from(value: io::Error) -> Self {
        LexError::GenericError(value.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frontend_error_debug() {
        let err = LexError::GenericError("test message".to_string());
        let debug_str = format!("{:?}", err);
        assert!(debug_str.contains("test message"));
    }

    #[test]
    fn test_frontend_error_display() {
        let err = LexError::GenericError("test message".to_string());
        let display_str = format!("{}", err);
        assert_eq!(display_str, "test message");
    }
}
