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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frontend_error_debug() {
        let err = LexError::FrontendError("test message".to_string());
        let debug_str = format!("{:?}", err);
        assert!(debug_str.contains("FrontendError"));
        assert!(debug_str.contains("test message"));
    }

    #[test]
    fn test_frontend_error_display() {
        let err = LexError::FrontendError("test message".to_string());
        let display_str = format!("{}", err);
        assert_eq!(display_str, "Frontend error: test message");
    }
}
