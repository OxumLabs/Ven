use std::str;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    Comment,    // ';' (comment, consumes until newline)
    At,         // '@' (variable declaration)
    Greater,    // '>' (input or print operator)
    Plus,       // '+' (addition operator)
    Minus,      // '-' (subtraction operator)
    Star,       // '*' (multiplication operator)
    Slash,      // '/' (division operator)
    Identifier, // Alphanumeric identifiers (including type keywords and literals)
    Newline,    // '\n' (statement separator)
    Unknown,    // Any unknown character
}

#[derive(Debug, Clone, Copy)]
pub struct Token {
    pub kind: TokenKind,
    pub start: usize,
    pub end: usize,
}

/// Ultra-fast tokenizer with minimal overhead.
pub struct Tokenizer<'a> {
    pub input: &'a [u8], // using byte slices for maximum speed
    pub pos: usize,
    pub tokens: Vec<Token>,
}

impl<'a> Tokenizer<'a> {
    #[inline(always)]
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input.as_bytes(),
            pos: 0,
            tokens: Vec::with_capacity(input.len() / 2), // heuristic preallocation
        }
    }

    #[inline(always)]
    pub fn tokenize(&mut self) -> &[Token] {
        let len = self.input.len();
        let bytes = self.input;
        let mut pos = self.pos;
        
        while pos < len {
            let start = pos;
            let kind = match bytes[pos] {
                b';' => {
                    pos += 1;
                    while pos < len && bytes[pos] != b'\n' {
                        pos += 1;
                    }
                    TokenKind::Comment
                }
                b'@' => {
                    pos += 1;
                    TokenKind::At
                }
                b'>' => {
                    pos += 1;
                    TokenKind::Greater
                }
                b'+' => {
                    pos += 1;
                    TokenKind::Plus
                }
                b'-' => {
                    pos += 1;
                    TokenKind::Minus
                }
                b'*' => {
                    pos += 1;
                    TokenKind::Star
                }
                b'/' => {
                    pos += 1;
                    TokenKind::Slash
                }
                b'\n' => {
                    pos += 1;
                    TokenKind::Newline
                }
                // Recognize alphanumeric, quotes, and brackets as Identifier
                b'0'..=b'9' | b'A'..=b'Z' | b'a'..=b'z' | b'"' | b'[' | b']' => {
                    while pos < len &&
                        matches!(bytes[pos], b'0'..=b'9' | b'A'..=b'Z' | b'a'..=b'z' | b'"' | b'[' | b']')
                    {
                        pos += 1;
                    }
                    TokenKind::Identifier
                }
                _ => {
                    pos += 1;
                    TokenKind::Unknown
                }
            };
            self.tokens.push(Token { kind, start, end: pos });
        }
        self.tokens.as_slice()
    }

    #[inline(always)]
    pub fn debug(&self) {
        println!("Token Tree:");
        let mut start = 0;
        let len = self.tokens.len();
        let input = self.input;
        for i in 0..len {
            let token = self.tokens[i];
            if input[token.start] == b'\n' {
                self.print_tokens(&self.tokens[start..=i]);
                start = i + 1;
            }
        }
        if start < len {
            self.print_tokens(&self.tokens[start..]);
        }
        println!("End of Token Tree");
    }

    #[inline(always)]
    fn print_tokens(&self, group: &[Token]) {
        if group.is_empty() { return; }
        println!("├── Statement");
        for (i, token) in group.iter().enumerate() {
            let lexeme = unsafe {
                std::str::from_utf8_unchecked(&self.input[token.start..token.end])
            };
            let connector = if i == group.len() - 1 { "    └──" } else { "    ├──" };
            println!("{} [{:?}] \"{}\" ({}..{})", connector, token.kind, lexeme, token.start, token.end);
        }
    }
}
