pub fn tokenizer_error(file: &str, line: usize, msg: &str) {
    println!("{}:{} {}", file, line, msg)
}

pub fn parser_error(msg: &str) {
    println!("error parsing: {}", msg)
}

pub fn parser_error_str(msg: String) {
    println!("error parsing: {}", msg)
}
