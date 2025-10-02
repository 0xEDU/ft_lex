pub struct Cursor<'a> {
    content: &'a [u8],
    current_position: usize,
    pub line_number: usize,
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
            return None;
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
