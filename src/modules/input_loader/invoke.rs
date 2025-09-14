use std::env;

use crate::{modules::input_loader::options::Options, shared::LexError};

pub fn invoke() -> Result<String, LexError> {
    let arguments = env::args().skip(1);
    let options = Options::from_arguments(arguments)?;

    Ok("dale".to_string())
}
