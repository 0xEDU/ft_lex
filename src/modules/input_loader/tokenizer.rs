use std::fs::{self};

use crate::shared::LexError;

struct Cursor<'a> {
    content: &'a [u8],
    current_position: usize,
    line_number: usize,
    current_line: Vec<u8>
}

impl<'a> Cursor<'a> {
    pub fn new(content: &'a str) -> Self {
        let content: &'a [u8] = content.as_bytes();
        Cursor { 
            content,
            current_position: 0,
            line_number: 0,
            current_line: Vec::new(),
         }
    }

    pub fn peek(&self) -> Option<&u8> {
        self.content.get(self.current_position)
    }

    pub fn is_at_end(&self) -> bool {
        self.current_position >= self.content.len()
    }

    pub fn consume_one(&mut self) -> Option<&u8> {
        self.consume(1)
    }

    pub fn consume(&mut self, quantity: usize) -> Option<&u8> {
        let byte = self.content.get(self.current_position);
        self.current_position += quantity;
        return byte;
    }

    pub fn is_at_str(&self, s: &str) -> bool {
        let end = self.current_position + s.len();
        end <= self.content.len() && &self.content[self.current_position..end] == s.as_bytes()
    }

    pub fn is_current_line_empty(&self) -> bool {
        self.current_line.len() == 0
    }

    pub fn line_starts_with(&self, s: &str) -> bool {
        let len = s.len();
        len <= self.content.len()
            && len <= self.current_line.len()
            && &self.current_line[0..len] == s.as_bytes()
    }

    pub fn get_next_line(&mut self) {
        self.current_line.clear();
        while !self.is_at_end() && !matches!(self.peek(), Some(b'\n')) {
            match self.consume_one() {
                Some(&c) => self.current_line.push(c),
                None => println!("this should be unreachable")
            }
        }
        if matches!(self.peek(), Some(b'\n')) {
            self.consume_one();
            self.line_number += 1;
        }
    }
}

#[derive(Debug)]
enum TokenizerState {
    Definitions,
    Rules,
    UserSubroutines,
}

fn tokenize_operand(operand: String) -> Result<(), LexError> {
    let content = fs::read_to_string(operand)?;
    let mut cursor = Cursor::new(&content);
    let mut state = TokenizerState::Definitions;

    while !cursor.is_at_end() {
        cursor.get_next_line();
        if cursor.is_current_line_empty() {
            continue;
        }

        match state {
            TokenizerState::Definitions => {
                if cursor.line_starts_with(" ") {
                    print!("line:{}:", cursor.line_number);
                    println!("found C code line");
                    // copy the C code line
                    continue;
                }
                if cursor.line_starts_with("%%") {
                    print!("line:{}:", cursor.line_number);
                    println!("found a section delimiter");
                    state = TokenizerState::Rules; // hop to next state
                    continue;
                }
                if cursor.line_starts_with("%{") {
                    print!("line:{}:", cursor.line_number);
                    println!("found a code block");
                    // custom function to consume entire code block
                    continue;
                }
                if cursor.line_starts_with("%option") {
                    print!("line:{}:", cursor.line_number);
                    println!("found a start option");
                    // copy start options
                    continue;
                }
                if cursor.line_starts_with("%s") || cursor.line_starts_with("%x") {
                    print!("line:{}:", cursor.line_number);
                    println!("found a start condition");
                    // copy start options
                    continue;
                }
                if cursor.peek().is_some() {
                    print!("line:{}:", cursor.line_number);
                    println!("found a macro");
                    // handle macros
                    continue;
                }
            },
            TokenizerState::Rules => println!("rules"),
            TokenizerState::UserSubroutines => println!("user subroutes")
        }
    }
    return Ok(());
}

pub fn tokenize_operands(operands: Vec<String>) -> Result<(), LexError> {
    for operand in operands {
        tokenize_operand(operand)?;
    }
    return Ok(());
}
