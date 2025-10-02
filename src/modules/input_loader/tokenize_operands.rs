use std::fs::{self};

use crate::{
    modules::input_loader::cursor::Cursor,
    shared::{logger, LexError},
};

#[derive(Debug)]
enum TokenizerState {
    Definitions,
    Rules,
    UserSubroutines,
}

// Result<Vec<Token>, LexError>
fn tokenize_operand(operand: String) -> Result<(), LexError> {
    let content = fs::read_to_string(&operand)?;
    let mut cursor = Cursor::new(&content);
    let mut state = TokenizerState::Definitions;

    while let Some(line) = cursor.next_line() {
        if line.is_empty() {
            continue;
        }

        match state {
            TokenizerState::Definitions => {
                if line.starts_with(b" ") || line.starts_with(b"\t") {
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
                    let mut code_block: Vec<u8> = Vec::new();
                    while let Some(inner) = cursor.next_line() {
                        if inner.starts_with(b"%}") {
                            break;
                        }
                        code_block.extend_from_slice(&inner);
                        code_block.push(b'\n');
                    }
                    print!("line:{}:", cursor.line_number);
                    println!("finish chomping");
                    continue;
                }
                if line.starts_with(b"/*") {
                    print!("line:{}:", cursor.line_number);
                    println!("found a comment block, chomping...");
                    let mut comment_block: Vec<u8> = Vec::new();
                    while let Some(inner) = cursor.next_line() {
                        if inner.ends_with(b"*/") {
                            break;
                        }
                        comment_block.extend_from_slice(&inner);
                        comment_block.push(b'\n');
                    }
                    print!("line:{}:", cursor.line_number);
                    println!("finish chomping");
                    continue;
                }
                // Opts are: pnaeko sx array/pointer
                if line.starts_with(b"%s ") {
                    print!("line:{}:", cursor.line_number);
                    println!("found a start condition");
                    // copy start options
                    continue;
                }
                if line.starts_with(b"%x ") {
                    print!("line:{}:", cursor.line_number);
                    println!("found an exclusive start condition");
                    // copy start options
                    continue;
                }
                if line.starts_with(b"%array") {
                    print!("line:{}:", cursor.line_number);
                    println!("found an array scanner option");
                    // copy start options
                    continue;
                }
                if line.starts_with(b"%pointer") {
                    print!("line:{}:", cursor.line_number);
                    println!("found a pointer scanner option");
                    // copy start options
                    continue;
                }
                if line.starts_with(b"%p ")
                    || line.starts_with(b"%n ")
                    || line.starts_with(b"%a ")
                    || line.starts_with(b"%e ")
                    || line.starts_with(b"%k ")
                    || line.starts_with(b"%o ")
                {
                    print!("line:{}:", cursor.line_number);
                    println!("found a table size option");
                    continue;
                }
                if line.starts_with(b"%") {
                    logger::tokenizer_error(&operand, cursor.line_number, "malformed option");
                    continue;
                }
                if !line.is_empty() {
                    print!("line:{}:", cursor.line_number);
                    println!("found a macro definition");
                    // handle macros
                    continue;
                }
            }
            TokenizerState::Rules => {
                if line.starts_with(b"%%") {
                    print!("line:{}:", cursor.line_number);
                    println!("found a section delimiter");
                    state = TokenizerState::UserSubroutines; // hop to next state
                    continue;
                } else {
                    print!("line:{}:", cursor.line_number);
                    println!("this is a rule");
                    continue;
                }
            }
            TokenizerState::UserSubroutines => {
                print!("line:{}:", cursor.line_number);
                println!("this line will be copied verbatim");
                continue;
            }
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
