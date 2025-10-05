use std::fs::{self};

use crate::{
    modules::input_loader::{
        cursor::Cursor,
        tokens::{ScannerStorageKind, Token},
    },
    shared::{lex_error, logger, LexError},
};

#[derive(Debug)]
enum TokenizerState {
    Definitions,
    Rules,
    UserSubroutines,
}

// Result<Vec<Token>, LexError>
fn tokenize_operand(operand: String) -> Result<Vec<Token>, LexError> {
    let content = fs::read_to_string(&operand)?;
    let mut cursor = Cursor::new(&content);
    let mut state = TokenizerState::Definitions;
    let mut tokens: Vec<Token> = Vec::new();

    while let Some(line) = cursor.next_line() {
        if line.is_empty() {
            continue;
        }

        match state {
            TokenizerState::Definitions => {
                if line.starts_with(b" ") || line.starts_with(b"\t") {
                    tokens.push(Token::CodeLine(line));
                    continue;
                }
                if line.starts_with(b"%%") {
                    state = TokenizerState::Rules; // hop to next state
                    continue;
                }
                if line.starts_with(b"%{") {
                    let mut code_block: Vec<u8> = Vec::new();
                    while let Some(inner) = cursor.next_line() {
                        if inner.starts_with(b"%}") {
                            break;
                        }
                        code_block.extend_from_slice(&inner);
                        code_block.push(b'\n');
                    }
                    tokens.push(Token::CodeBlock(code_block));
                    continue;
                }
                if line.starts_with(b"/*") {
                    while let Some(inner) = cursor.next_line() {
                        if inner.ends_with(b"*/") {
                            break;
                        }
                    }
                    continue;
                }
                // Opts are: pnaeko sx array/pointer
                if line.starts_with(b"%s ") {
                    tokens.push(Token::StartConditionInclusive(line));
                    continue;
                }
                if line.starts_with(b"%x ") {
                    tokens.push(Token::StartConditionExclusive(line));
                    continue;
                }
                if line.starts_with(b"%array") {
                    tokens.push(Token::ScannerStorage(ScannerStorageKind::Array));
                    continue;
                }
                if line.starts_with(b"%pointer") {
                    tokens.push(Token::ScannerStorage(ScannerStorageKind::Pointer));
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
                    return Err(LexError::GenericError("invalid_token".to_string()))
                }
                if !line.is_empty() {
                    tokens.push(Token::MacroDefinition(line));
                    continue;
                }
            }
            TokenizerState::Rules => {
                if line.starts_with(b"%%") {
                    state = TokenizerState::UserSubroutines; // hop to next state
                    continue;
                } else {
                    tokens.push(Token::Rule(line));
                    continue;
                }
            }
            TokenizerState::UserSubroutines => {
                tokens.push(Token::UserSubroutine(line));
                continue;
            }
        }
    }
    return Ok(tokens);
}

pub fn tokenize_operands(operands: Vec<String>) -> Result<Vec<Token>, LexError> {
    let mut tokens: Vec<Token> = Vec::new();

    for operand in operands {
        let mut operand_tokens = tokenize_operand(operand)?;
        tokens.append(&mut operand_tokens);
    }
    return Ok(tokens);
}
