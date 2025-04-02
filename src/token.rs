use std::str;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    Comment,    // ';' (comment, consumes until newline)
    At,         // '@' (variable declaration)
    Greater,    // '>' (print operator)
    DoubleDot,  // '..' (input operator)
    Plus,       // '+' (addition operator)
    Minus,      // '-' (subtraction operator)
    Star,       // '*' (multiplication operator)
    Slash,      // '/' (division operator)
    Identifier, // Alphanumeric identifiers (including type keywords and literals)
    Newline,    // '\n' (statement separator)
    Unknown,    // Any unknown character
    RSmallB,    // Right Small Bracket
    LSmallB,    // Left Small Bracket
    RBigB,      // Right Big Bracket
    LBigB,      // Left Big Bracket
    RCurlyB,    // Right Curly Bracket
    LCurlyB,    // Left Curly Bracket
    Question,   // '?' (conditional operator)
    Equals,     // '=' (assignment operator)
    Equal,      // '==' (equality operator)
    NotEqual,   // '!=' (inequality operator)
    LessThan,   // '<' (less than operator)
    LessEqual,  // '<=' (less than or equal operator)
    GreaterThan,// '>' (greater than operator - overlap with Greater, disambiguated in parsing)
    GreaterEqual,// '>=' (greater than or equal operator)
    And,        // '&&' (logical AND operator)
    Or,         // '||' (logical OR operator)
    StringLiteral, // String literal enclosed in double quotes
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
                b'{' => {
                    pos += 1;
                    TokenKind::LCurlyB
                }
                b'}' => {
                    pos += 1;
                    TokenKind::RCurlyB
                }
                b'[' => {
                    pos += 1;
                    TokenKind::LBigB
                }
                b']' => {
                    pos += 1;
                    TokenKind::RBigB
                }
                b'(' => {
                    pos += 1;
                    TokenKind::LSmallB
                }
                b')' => {
                    pos += 1;
                    TokenKind::RSmallB
                }
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
                    // Check for '>='
                    if pos < len && bytes[pos] == b'=' {
                        pos += 1;
                        TokenKind::GreaterEqual
                    } else {
                        TokenKind::Greater
                    }
                }
                b'<' => {
                    pos += 1;
                    // Check for '<='
                    if pos < len && bytes[pos] == b'=' {
                        pos += 1;
                        TokenKind::LessEqual
                    } else {
                        TokenKind::LessThan
                    }
                }
                b'=' => {
                    pos += 1;
                    // Check for '=='
                    if pos < len && bytes[pos] == b'=' {
                        pos += 1;
                        TokenKind::Equal
                    } else {
                        TokenKind::Equals
                    }
                }
                b'!' => {
                    pos += 1;
                    // Check for '!='
                    if pos < len && bytes[pos] == b'=' {
                        pos += 1;
                        TokenKind::NotEqual
                    } else {
                        TokenKind::Unknown
                    }
                }
                b'&' => {
                    pos += 1;
                    // Check for '&&'
                    if pos < len && bytes[pos] == b'&' {
                        pos += 1;
                        TokenKind::And
                    } else {
                        TokenKind::Unknown
                    }
                }
                b'|' => {
                    pos += 1;
                    // Check for '||'
                    if pos < len && bytes[pos] == b'|' {
                        pos += 1;
                        TokenKind::Or
                    } else {
                        TokenKind::Unknown
                    }
                }
                b'?' => {
                    pos += 1;
                    TokenKind::Question
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
                b'.' => {
                    pos += 1;
                    // Check for '..'
                    if pos < len && bytes[pos] == b'.' {
                        pos += 1;
                        TokenKind::DoubleDot
                    } else {
                        TokenKind::Unknown // Single dot isn't recognized
                    }
                }
                // Handle string literals
                b'"' => {
                    pos += 1; // Skip the opening quote
                    let _string_start = pos;
                    // Consume everything until the closing quote or end of line
                    while pos < len && bytes[pos] != b'"' && bytes[pos] != b'\n' {
                        pos += 1;
                    }
                    // If we found a closing quote, consume it
                    if pos < len && bytes[pos] == b'"' {
                        pos += 1;
                    }
                    TokenKind::StringLiteral
                }
                // Recognize alphanumeric identifiers
                b'0'..=b'9' | b'A'..=b'Z' | b'a'..=b'z' => {
                    while pos < len
                        && matches!(bytes[pos], b'0'..=b'9' | b'A'..=b'Z' | b'a'..=b'z' | b'_')
                    {
                        pos += 1;
                    }
                    TokenKind::Identifier
                }
                b'_' => {
                    pos += 1;
                    while pos < len
                        && matches!(bytes[pos], b'0'..=b'9' | b'A'..=b'Z' | b'a'..=b'z' | b'_')
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
            self.tokens.push(Token {
                kind,
                start,
                end: pos,
            });
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
        if group.is_empty() {
            return;
        }
        println!("├── Statement");
        for (i, token) in group.iter().enumerate() {
            let lexeme =
                unsafe { std::str::from_utf8_unchecked(&self.input[token.start..token.end]) };
            let connector = if i == group.len() - 1 {
                "    └──"
            } else {
                "    ├──"
            };
            println!(
                "{} [{:?}] \"{}\" ({}..{})",
                connector, token.kind, lexeme, token.start, token.end
            );
        }
    }
}
