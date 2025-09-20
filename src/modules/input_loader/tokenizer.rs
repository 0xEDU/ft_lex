use std::fs::{self};

use crate::shared::LexError;

#[derive(Debug)]
enum TokenizerState {
    Definitions,
    Rules,
    UserSubroutines,
}

pub struct Tokenizer {
    state: TokenizerState,
}

impl Tokenizer {
    pub fn new() -> Self {
        Tokenizer {
            state: TokenizerState::Definitions,
        }
    }

    fn next_state(&mut self) {
        match self.state {
            TokenizerState::Definitions => self.state = TokenizerState::Rules,
            TokenizerState::Rules => self.state = TokenizerState::UserSubroutines,
            _ => {}
        }
    }

    fn tokenize_definitions(&mut self) {
    }

    pub fn tokenize_operands(&mut self, operands: Vec<String>) -> Result<(), LexError> {
        for operand in operands {
            let content = fs::read_to_string(operand)?;

            match self.state {
                TokenizerState::Definitions => self.tokenize_definitions(),
                TokenizerState::Rules => println!("rules"),
                TokenizerState::UserSubroutines => println!("user subroutes")
                
            }
        }
        return Ok(());
    }
}
