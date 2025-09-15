use std::{fs::File, io::{BufRead, BufReader}};

use crate::shared::LexError;

pub fn tokenize_operands(operands: Vec<String>) -> Result<(), LexError> {
    for operand in operands {
        let file = match File::open(operand) {
            Ok(f) => f,
            Err(_) => return Err(LexError::FrontendError("Couldn't open file.".to_string())),
        };

        let buf_reader = BufReader::new(file);
        for line in buf_reader.lines() {
            match line {
                Ok(line) => println!("{}", line),
                Err(_) => println!("oops")
            }
        }
    }
    return Ok(());
}
