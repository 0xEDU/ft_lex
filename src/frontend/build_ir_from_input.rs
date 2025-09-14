use std::env;

use crate::{frontend::options::Options, shared::lex_error::LexError};

pub fn build_ir_from_input() -> Result<String, LexError> {
    let arguments = env::args().skip(1);
    let options = Options::from_arguments(arguments)?;

    Ok("dale".to_string())
}
