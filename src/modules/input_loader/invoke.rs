use std::env;

use crate::{
    modules::input_loader::{
        options::Options, parse_tokens::parse_tokens, tokenize_operands::tokenize_operands,
    },
    shared::LexError,
};

pub fn invoke() -> Result<String, LexError> {
    let arguments = env::args().skip(1);
    let options = Options::from_arguments(arguments)?;
    let tokens = tokenize_operands(options.operands)?;
    let _ = parse_tokens(tokens);

    Ok("dale".to_string())
}
