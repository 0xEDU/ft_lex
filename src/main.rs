mod frontend;
mod lex_error;

use crate::lex_error::LexError;

fn main() -> Result<(), LexError> {
    let intermediate_representation = frontend::build_ir_from_input()?;
    Ok(())
}
