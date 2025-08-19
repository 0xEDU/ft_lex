mod lex_error;

use crate::lex_error::LexError;
use std::env;

fn main() -> Result<(), LexError> {
    let file_name = env::args()
        .nth(1)
        .ok_or_else(|| LexError::FrontendError("File not found!".to_string()))?;

    println!("{}", file_name);
    Ok(())
}
