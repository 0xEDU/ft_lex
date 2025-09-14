mod modules;
mod shared;

use crate::shared::LexError;

fn main() -> Result<(), LexError> {
    let intermediate_representation = modules::input_loader::build_ir_from_input()?;
    Ok(())
}
