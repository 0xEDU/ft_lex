mod modules;
mod shared;

use crate::shared::LexError;
use modules::input_loader;

fn main() -> Result<(), LexError> {
    let intermediate_representation = input_loader::invoke();
    Ok(())
}
