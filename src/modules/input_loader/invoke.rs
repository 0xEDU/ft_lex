use std::env;

use crate::{
    modules::input_loader::{options::Options, tokenize_operands::tokenize_operands},
    shared::LexError,
};

pub fn invoke() -> Result<String, LexError> {
    let arguments = env::args().skip(1);
    let options = Options::from_arguments(arguments)?;
    let files = tokenize_operands(options.operands);

    Ok("dale".to_string())
}
