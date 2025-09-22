use std::fs::{self};

use crate::shared::LexError;

struct Cursor<'a> {
    content: &'a [u8],
    current: usize,
    line: usize,
}

impl<'a> Cursor<'a> {
    pub fn new(content: &'a str) -> Self {
        let content: &'a [u8] = content.as_bytes();
        Cursor { 
            content,
            current: 0,
            line: 0,
         }
    }

    pub fn peek(&self) -> Option<&u8> {
        self.content.get(self.current)
    }

    pub fn is_at_end(&self) -> bool {
        self.current >= self.content.len()
    }

    pub fn consume_one(&mut self) -> Option<&u8> {
        self.consume(1)
    }

    pub fn consume(&mut self, quantity: usize) -> Option<&u8> {
        let byte = self.content.get(self.current);
        self.current += quantity;
        return byte;
    }

    pub fn is_at_str(&self, s: &str) -> bool {
        let end = self.current + s.len();
        end <= self.content.len() && &self.content[self.current..end] == s.as_bytes()
    }
}

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

    fn tokenize_operand(&mut self, operand: String) -> Result<(), LexError> {
        let content = fs::read_to_string(operand)?;
        let mut cursor = Cursor::new(&content);
        let state = TokenizerState::Definitions;

        while !cursor.is_at_end() {
            if matches!(cursor.peek(), Some(b'\n')) {
                println!("");
                cursor.line += 1;
                cursor.consume_one();
            }

            // should skip tabs and stuff?

            match state {
                TokenizerState::Definitions => {
                    print!("{}", *cursor.peek().unwrap() as char);
                    if cursor.is_at_str("%%") {
                        println!("found a section delimiter");
                        cursor.consume(2);
                        continue;
                    }
                    if cursor.is_at_str("%{") {
                        cursor.consume(2);
                        println!("found a code block");
                        // custom function to conseume entire code block
                    }
                },
                TokenizerState::Rules => println!("rules"),
                TokenizerState::UserSubroutines => println!("user subroutes")
                
            }
            cursor.consume_one();
        }
        return Ok(());
    }

    pub fn tokenize_operands(&mut self, operands: Vec<String>) -> Result<(), LexError> {
        for operand in operands {
            self.tokenize_operand(operand)?;
        }
        return Ok(());
    }
}
