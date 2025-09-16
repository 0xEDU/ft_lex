use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::shared::LexError;

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

    fn tokenize_definitions(line: String) {}

    pub fn tokenize_operands(&self, operands: Vec<String>) -> Result<(), LexError> {
        for operand in operands {
            let file = File::open(operand)?;
            let buf_reader = BufReader::new(file);

            for line in buf_reader.lines() {
                let line = line?;
                if line.len() == 0 {
                    continue;
                }

                match self.state {
                    TokenizerState::Definitions => Self::tokenize_definitions(line),
                    TokenizerState::Rules => {}
                    TokenizerState::UserSubroutines => {}
                }
            }
        }
        return Ok(());
    }
}
