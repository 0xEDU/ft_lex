pub fn tokenizer_error(file: &str, line: usize, msg: &str) {
    println!("{}:{} {}", file, line, msg)
}

pub fn parser_error(msg: &str) {
    println!("invalid condition: {}", msg)
}