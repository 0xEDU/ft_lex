use std::{
    fs::File,
    io::{BufRead, BufReader},
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
    is_inside_code_block: bool,
}

impl Tokenizer {
    pub fn new() -> Self {
        Tokenizer {
            state: TokenizerState::Definitions,
            is_inside_code_block: false,
        }
    }

    fn next(&mut self) {
        match self.state {
            TokenizerState::Definitions => {
                self.state = TokenizerState::Rules;
            }
            TokenizerState::Rules => self.state = TokenizerState::UserSubroutines,
            _ => {}
        }
    }

    fn tokenize_definitions(&mut self, line: String) {
        if self.is_inside_code_block {
            if line.starts_with("%}") {
                println!("code block end");
                self.is_inside_code_block = false;
                return;
            }
            println!("line inside code block!");
            return;
        }

        let mut iter = line.chars();
        match iter.next() {
            Some(' ' | '\t') => {
                println!("found a line of C code")
            }
            Some('%') => match iter.next() {
                Some('{') => {
                    println!("code block start");
                    self.is_inside_code_block = true;
                }
                Some(c) => {
                    println!("start condition")
                } // Branch out valid start conditions
                None => {
                    println!("syntax error")
                }
            },
            Some(_) => {
                println!("name defition")
            }
            None => {
                println!("blank line, shouldn't reach this")
            }
        }
    }

    pub fn tokenize_operands(&mut self, operands: Vec<String>) -> Result<(), LexError> {
        for operand in operands {
            let file = File::open(operand)?;
            let buf_reader = BufReader::new(file);
            let mut current_line = 1;

            for line in buf_reader.lines() {
                let line = line?;
                if line.is_empty() {
                    current_line += 1;
                    continue;
                }

                if line == "%%" && !self.is_inside_code_block {
                    self.next();
                }

                print!("current line: {}: ", current_line);
                match self.state {
                    TokenizerState::Definitions => self.tokenize_definitions(line),
                    TokenizerState::Rules => {
                        println!("")
                    }
                    TokenizerState::UserSubroutines => {
                        println!("")
                    }
                }
                current_line += 1;
            }
        }
        return Ok(());
    }
}
