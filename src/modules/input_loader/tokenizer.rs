use std::fs::{self};

use crate::shared::LexError;

struct Cursor<'a> {
    content: &'a [u8],
    current_position: usize,
    line_number: usize,
}

impl<'a> Cursor<'a> {
    pub fn new(content: &'a str) -> Self {
        let content: &'a [u8] = content.as_bytes();
        Cursor { 
            content,
            current_position: 0,
            line_number: 0,
         }
    }

    pub fn peek(&self) -> Option<u8> {
        self.content.get(self.current_position).copied()
    }

    pub fn is_at_end(&self) -> bool {
        self.current_position >= self.content.len()
    }

    pub fn consume_one(&mut self) -> Option<u8> {
        let byte = self.peek();
        self.consume(1);
        byte
    }

    pub fn consume(&mut self, quantity: usize) {
        self.current_position = (self.current_position + quantity).min(self.content.len());
    }

    pub fn next_line(&mut self) -> Option<Vec<u8>> {
        if self.is_at_end() {
            return None
        }

        let mut line = Vec::new();
        while let Some(c) = self.consume_one() {
            if c == b'\n' {
                self.line_number += 1;
                break;
            }
            line.push(c);
        }
        
        Some(line)
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

    while let Some(line) = cursor.next_line() {
        if line.is_empty() {
            continue;
        }

        match state {
            TokenizerState::Definitions => {
                if line.starts_with(b" ") {
                    print!("line:{}:", cursor.line_number);
                    println!("found C code line");
                    // copy the C code line
                    continue;
                }
                if line.starts_with(b"%%") {
                    print!("line:{}:", cursor.line_number);
                    println!("found a section delimiter");
                    state = TokenizerState::Rules; // hop to next state
                    continue;
                }
                if line.starts_with(b"%{") {
                    print!("line:{}:", cursor.line_number);
                    println!("found a code block, chomping...");
                    let mut code_block : Vec<u8> = Vec::new();
                    while let Some(inner) = cursor.next_line() {
                        if inner.starts_with(b"%}") {
                            break;
                        }
                        code_block.extend_from_slice(&inner);
                        code_block.push(b'\n');
                    }
                    continue;
                }
                if line.starts_with(b"/*") {
                    print!("line:{}:", cursor.line_number);
                    println!("found a comment block, chomping...");
                    let mut comment_block : Vec<u8> = Vec::new();
                    while let Some(inner) = cursor.next_line() {
                        if inner.ends_with(b"*/") {
                            break;
                        }
                        comment_block.extend_from_slice(&inner);
                        comment_block.push(b'\n');
                    }
                    continue;
                }
                // Opts are: pnaeko sx array/pointer
                if line.starts_with(b"%s") || line.starts_with(b"%x") {
                    print!("line:{}:", cursor.line_number);
                    println!("found a start condition");
                    // copy start options
                    continue;
                }
                if !line.is_empty() {
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
