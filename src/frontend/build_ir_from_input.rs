use std::env;

use crate::lex_error::LexError;

pub fn build_ir_from_input() -> Result<String, LexError> {
    let arguments: Vec<String> = env::args().skip(1).collect();

    Ok("dale".to_string())
}
