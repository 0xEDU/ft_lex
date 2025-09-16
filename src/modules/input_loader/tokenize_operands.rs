use std::{fs::File, io::{BufRead, BufReader}};

use crate::shared::LexError;

enum TokenizerStates {
    Definitions,
    Rules,
    UserSubroutines
}

fn tokenize_definitions(line: String) {
    
}

pub fn tokenize_operands(operands: Vec<String>) -> Result<(), LexError> {
    for operand in operands {
        let file = File::open(operand)?;
        let buf_reader = BufReader::new(file);

        let current_state = TokenizerStates::Definitions;
        for line in buf_reader.lines() {
            let line = line?;

            match current_state {
                TokenizerStates::Definitions => tokenize_definitions(line),
                TokenizerStates::Rules => {},
                TokenizerStates::UserSubroutines => {},
            }
        }
    }
    return Ok(());
}
