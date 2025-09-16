use std::env;

use crate::{
    modules::input_loader::{options::Options, tokenizer::Tokenizer},
    shared::LexError,
};

pub fn invoke() -> Result<String, LexError> {
    let arguments = env::args().skip(1);
    let options = Options::from_arguments(arguments)?;
    let files = Tokenizer::new().tokenize_operands(options.operands);

    Ok("dale".to_string())
}
