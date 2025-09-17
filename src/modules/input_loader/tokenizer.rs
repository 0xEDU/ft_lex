use std::{
    fs::File, io::{BufRead, BufReader}
};

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

    fn next(&mut self) {
        match self.state {
            TokenizerState::Definitions => {
                self.state = TokenizerState::Rules;
            }
            TokenizerState::Rules => {
                self.state = TokenizerState::UserSubroutines
            }
            _ => {}
        }
    }

    fn tokenize_definitions(line: String) {
        let mut iter = line.chars();
        match iter.next() {
            Some(' ' | '\t' ) => {
                println!("found a line of C code")
            }
            Some('%') => {
                let next_char = iter.next();
                match next_char {
                    Some('{') => { println!("code block start") }
                    Some('}') => { println!("code block end") }
                    Some(c) => { println!("start condition") } // Branch out valid start conditions
                    None => { println!("syntax error") }
                }
            }
            Some(_) => { println!("name defition") }
            None => { println!("blank line, shouldn't reach this") }
        }
    }

    pub fn tokenize_operands(&mut self, operands: Vec<String>) -> Result<(), LexError> {
        for operand in operands {
            let file = File::open(operand)?;
            let buf_reader = BufReader::new(file);
            let mut current_line = 1;

            for line in buf_reader.lines() {
                let line = line?;
                if line.len() == 0 {
                    current_line += 1;
                    continue;
                }

                if line == "%%" {
                    self.next();
                }

                print!("current line: {}: ", current_line);
                match self.state {
                    TokenizerState::Definitions => Self::tokenize_definitions(line),
                    TokenizerState::Rules => { println!("") }
                    TokenizerState::UserSubroutines => { println!("") }
                }
                current_line += 1;
            }
        }
        return Ok(());
    }
}
