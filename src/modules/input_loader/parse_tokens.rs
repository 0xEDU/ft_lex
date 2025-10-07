use crate::{
    modules::input_loader::{lex_program::LexProgram, tokens::Token},
    shared::{logger, LexError},
};

fn is_valid_identifier(name: &[u8]) -> bool {
    if name.is_empty() {
        return false;
    }

    let first = name[0];
    if !(first == b'_' || first.is_ascii_alphabetic()) {
        return false;
    }

    name.iter()
        .skip(1)
        .all(|&c| c == b'_' || c.is_ascii_alphanumeric())
}

fn parse_macro(line: Vec<u8>) -> (Vec<u8>, Vec<u8>) {
    return (Vec::new(), Vec::new())
}

fn parse_start_condition(start_condition: Vec<u8>) -> Result<Vec<Vec<u8>>, LexError> {
    let conditions: Vec<Vec<u8>> = Vec::new();

    let names = start_condition
        .split(|&c| c == b' ' || c == b'\t')
        .skip(1)
        .filter(|s| !s.is_empty())
        .map(|s| s.to_vec())
        .collect::<Vec<_>>();

    for name in &names {
        if !is_valid_identifier(name) {
            logger::parser_error_str(format!(
                "invalid start condition {}",
                String::from_utf8_lossy(name)
            ));
            return Err(LexError::ParserError)
        }
    }

    Ok(conditions)
}

pub fn parse_tokens(tokens: Vec<Token>) -> Result<LexProgram, LexError> {
    let mut lex_program = LexProgram::new();

    for token in tokens {
        match token {
            Token::CodeLine(line) => {
                lex_program.prelude_code.extend_from_slice(&line);
                lex_program.prelude_code.push(b'\n');
            }
            Token::CodeBlock(line) => {
                lex_program.prelude_code.extend_from_slice(&line);
                lex_program.prelude_code.push(b'\n');
            }
            Token::StartConditionInclusive(start_condition) => {
                let parsed_start_condition = parse_start_condition(start_condition)?;
                lex_program
                    .start_condition_inclusive
                    .extend_from_slice(&parsed_start_condition);
            }
            Token::StartConditionExclusive(start_condition) => {
                let parsed_start_condition = parse_start_condition(start_condition)?;
                lex_program
                    .start_condition_exclusive
                    .extend_from_slice(&parsed_start_condition);
            }
            Token::ScannerStorage(kind) => lex_program.storage_kind = kind,
            Token::MacroDefinition(line) => {
                let macro_definition = parse_macro(line);
            },
            Token::UserSubroutine(line) => {
                lex_program.user_subroutines.extend_from_slice(&line);
                lex_program.user_subroutines.push(b'\n');
            }
            _ => {}
        }
    }

    Ok(lex_program)
}
