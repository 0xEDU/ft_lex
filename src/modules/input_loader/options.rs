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
                        return Err(LexError::FrontendError("Invalid argument".to_string()));
                    }
                    options.operands.push(operand.to_string())
                }
            }
        }
        Ok(options)
    }
}
