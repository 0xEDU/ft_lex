use std::env::Args;
use std::iter::Skip;

use crate::shared::LexError;

#[derive(Debug)]
pub struct Options {
    pub do_nothing: bool,
    pub use_stdout: bool,
    pub verbose: bool,
    pub operands: Vec<String>,
}

impl Options {
    fn default() -> Self {
        Self {
            do_nothing: false,
            use_stdout: false,
            verbose: false,
            operands: vec![],
        }
    }

    pub fn from_arguments(arguments: Skip<Args>) -> Result<Options, LexError> {
        Self::from_iterator(arguments)
    }

    fn from_iterator<I>(arguments: I) -> Result<Options, LexError>
    where
        I: IntoIterator<Item = String>,
    {
        let mut options = Options::default();
        for argument in arguments {
            match argument.as_str() {
                "-n" => {
                    options.do_nothing = true;
                    options.verbose = false;
                }
                "-t" => options.use_stdout = true,
                "-v" => {
                    options.do_nothing = false;
                    options.verbose = true;
                }
                operand => {
                    if operand.starts_with("-") && operand != "-" {
                        return Err(LexError::InputLoaderError("Invalid argument".to_string()));
                    }
                    options.operands.push(operand.to_string())
                }
            }
        }
        Ok(options)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_options() {
        let opts = Options::default();
        assert!(!opts.do_nothing);
        assert!(!opts.use_stdout);
        assert!(!opts.verbose);
        assert!(opts.operands.is_empty());
    }

    #[test]
    fn test_flag_n_sets_do_nothing_and_disables_verbose() {
        let args = vec!["-n".to_string()];
        let opts = Options::from_iterator(args).unwrap();
        assert!(opts.do_nothing);
        assert!(!opts.verbose);
        assert!(!opts.use_stdout);
        assert!(opts.operands.is_empty());
    }

    #[test]
    fn test_flag_t_sets_use_stdout() {
        let args = vec!["-t".to_string()];
        let opts = Options::from_iterator(args).unwrap();
        assert!(!opts.do_nothing);
        assert!(!opts.verbose);
        assert!(opts.use_stdout);
        assert!(opts.operands.is_empty());
    }

    #[test]
    fn test_flag_v_sets_verbose_and_disables_do_nothing() {
        let args = vec!["-v".to_string()];
        let opts = Options::from_iterator(args).unwrap();
        assert!(!opts.do_nothing);
        assert!(opts.verbose);
        assert!(!opts.use_stdout);
        assert!(opts.operands.is_empty());
    }

    #[test]
    fn test_operands_are_collected() {
        let args = vec!["file1.txt".to_string(), "file2.txt".to_string()];
        let opts = Options::from_iterator(args).unwrap();
        assert!(!opts.do_nothing);
        assert!(!opts.verbose);
        assert!(!opts.use_stdout);
        assert_eq!(opts.operands, vec!["file1.txt", "file2.txt"]);
    }

    #[test]
    fn test_dash_is_valid_operand() {
        let args = vec!["-".to_string()];
        let opts = Options::from_iterator(args).unwrap();
        assert_eq!(opts.operands, vec!["-"]);
    }

    #[test]
    fn test_mixed_flags_and_operands() {
        let args = vec![
            "-n".to_string(),
            "file.txt".to_string(),
            "-t".to_string(),
            "another.txt".to_string(),
        ];
        let opts = Options::from_iterator(args).unwrap();
        assert!(opts.do_nothing);
        assert!(!opts.verbose);
        assert!(opts.use_stdout);
        assert_eq!(opts.operands, vec!["file.txt", "another.txt"]);
    }

    #[test]
    fn test_n_flag_overrides_v_flag() {
        let args = vec!["-v".to_string(), "-n".to_string()];
        let opts = Options::from_iterator(args).unwrap();
        assert!(opts.do_nothing);
        assert!(!opts.verbose);
    }

    #[test]
    fn test_v_flag_overrides_n_flag() {
        let args = vec!["-n".to_string(), "-v".to_string()];
        let opts = Options::from_iterator(args).unwrap();
        assert!(!opts.do_nothing);
        assert!(opts.verbose);
    }

    #[test]
    fn test_invalid_flag_returns_error() {
        let args = vec!["--invalid".to_string()];
        let result = Options::from_iterator(args);
        assert!(result.is_err());
        match result.unwrap_err() {
            LexError::InputLoaderError(msg) => assert_eq!(msg, "Invalid argument"),
        }
    }

    #[test]
    fn test_invalid_short_flag_returns_error() {
        let args = vec!["-x".to_string()];
        let result = Options::from_iterator(args);
        assert!(result.is_err());
        match result.unwrap_err() {
            LexError::InputLoaderError(msg) => assert_eq!(msg, "Invalid argument"),
        }
    }

    #[test]
    fn test_empty_arguments() {
        let args: Vec<String> = vec![];
        let opts = Options::from_iterator(args).unwrap();
        assert!(!opts.do_nothing);
        assert!(!opts.verbose);
        assert!(!opts.use_stdout);
        assert!(opts.operands.is_empty());
    }
}
