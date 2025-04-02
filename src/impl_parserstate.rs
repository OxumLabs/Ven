use std::collections::HashMap;

use crate::{errs::VarError, parse::*, parse1::parse1, parse2::parse2, token::{Token, TokenKind}};


pub struct ParserState<'a> {
    pub tokens: &'a [Token],
    pub input: &'a str,
    pub pos: usize,
    pub global_var_map: &'a mut HashMap<String, (VarType, usize)>,
    pub errors: Vec<VarError>,
    pub global_pos: usize, // This is the global position tracker
}

impl<'a> ParserState<'a> {
    #[inline(always)]
    fn current_token(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    #[inline(always)]
    fn peek_token(&self, n: usize) -> Option<&Token> {
        self.tokens.get(self.pos + n)
    }

    #[inline(always)]
    fn get_lexeme(&self, token: &Token) -> &str {
        // SAFETY: the token indices are assumed to be valid.
        unsafe { std::str::from_utf8_unchecked(&self.input.as_bytes()[token.start..token.end]) }
    }

    #[inline(always)]
    fn skip_whitespace(&mut self) {
        println!("DEBUG SKIP: Before skip_whitespace pos={}", self.pos);
        while self.pos < self.tokens.len() {
            let token = &self.tokens[self.pos];
            if token.kind == TokenKind::Unknown && self.get_lexeme(token).trim().is_empty() {
                self.pos += 1;
            } else {
                break;
            }
        }
        println!("DEBUG SKIP: After skip_whitespace pos={}", self.pos);
    }

    #[inline(always)]
    fn count_consecutive_greater(&self) -> usize {
        let mut count = 0;
        let mut pos = self.pos;
        while pos < self.tokens.len() {
            let token = &self.tokens[pos];
            if self.get_lexeme(token) == ">" {
                count += 1;
                pos += 1;
            } else {
                break;
            }
        }
        count
    }

    // In your ParserState implementation:
    #[inline(always)]
    pub fn parse_statement(&mut self, all_tokens: &[Token]) -> Option<ASTNode> {
        self.skip_whitespace();
        
        // Special handling for DoubleDot token at the correct position
        if self.pos < self.tokens.len() && self.tokens[self.pos].kind == TokenKind::DoubleDot {
            println!("DEBUG: Found DoubleDot at position {}", self.pos);
            self.pos += 1; // Skip the ".." token
            self.skip_whitespace();
            
            if let Some(name_token) = self.current_token() {
                if name_token.kind == TokenKind::Identifier {
                    let name = self.get_lexeme(name_token).to_string();
                    println!("DEBUG: Input variable name: {}", name);
                    self.pos += 1;
                    self.consume_until_newline();
                    
                    return Some(ASTNode::Input { name });
                }
            }
            
            return None;
        }
        
        if let Some(token) = self.current_token() {
            let token_kind = token.kind;
            let lexeme = self.get_lexeme(token);
            
            println!("DEBUG STMT: Parsing token with kind {:?} and lexeme \"{}\"", token_kind, lexeme);
            
            // Handle different statement types based on token kind or lexeme
            match token_kind {
                TokenKind::Question => {
                    println!("DEBUG STMT: Found Question token");
                    // This is a conditional statement
                    if let Some(cond_node) = parse2(all_tokens, self.pos, self.input, self) {
                        return Some(cond_node);
                    }
                    return None;
                },
                TokenKind::Star => {
                    println!("DEBUG STMT: Found Star token");
                    // This is a math operation
                    let token_clone = token.clone();
                    self.pos += 1; // consume "*" token
                    if let Some(math_node) = parse1(&self.tokens[self.pos..], self.input, &token_clone) {
                        self.consume_until_newline();
                        return Some(math_node);
                    }
                    return None;
                },
                TokenKind::At => {
                    println!("DEBUG STMT: Found At token (variable declaration)");
                    // This is a variable declaration
                    return self.parse_var_declaration();
                },
                TokenKind::Greater => {
                    println!("DEBUG STMT: Found Greater token (print operation)");
                    // This is a print operation
                    let count = self.count_consecutive_greater();
                    return self.parse_print(count);
                },
                _ => {
                    println!("DEBUG STMT: Unknown token kind: {:?}", token_kind);
                    // For all other token types, skip and continue
                    self.pos += 1;
                    return None;
                }
            }
        }
        None
    }

    #[inline(always)]
    fn parse_var_declaration(&mut self) -> Option<ASTNode> {
        // Check if the variable is mutable
        let mut mutable = false;
        if let Some(next) = self.peek_token(1) {
            if self.get_lexeme(next) == "@" {
                mutable = true;
                self.pos += 1; // Consume the second '@'
            }
        }

        self.pos += 1; // Consume '@'
        self.skip_whitespace();

        // Parse variable name
        let name_token = self.current_token()?;
        if name_token.kind != TokenKind::Identifier {
            return None;
        }
        let name = self.get_lexeme(name_token).to_string();
        self.pos += 1;
        self.skip_whitespace();

        // Parse type
        let type_token = self.current_token()?;
        let type_lex = self.get_lexeme(type_token);
        let var_type = match type_lex {
            "i" => {
                self.pos += 1;
                VarType::Int
            }
            "str" => {
                self.pos += 1;
                VarType::String
            }
            "f" => {
                self.pos += 1;
                VarType::Float
            }
            "c" => {
                self.pos += 1;
                self.skip_whitespace();
                if let Some(size_token) = self.current_token() {
                    if self.get_lexeme(size_token) == "[" {
                        self.pos += 1;
                        self.skip_whitespace();
                        let size_token = self.current_token()?;
                        let size_str = self.get_lexeme(size_token);
                        let size: usize = size_str.parse().unwrap_or(1);
                        self.pos += 1;
                        self.skip_whitespace();
                        if let Some(close_bracket) = self.current_token() {
                            if self.get_lexeme(close_bracket) == "]" {
                                self.pos += 1;
                                VarType::Char { size }
                            } else {
                                return None;
                            }
                        } else {
                            return None;
                        }
                    } else {
                        return None;
                    }
                } else {
                    return None;
                }
            }
            _ => return None,
        };
        self.skip_whitespace();

        // Parse optional initializer
        let mut value = self.parse_expression_until_newline();

        // Ensure string literals have proper quotes stripped
        if let Some(Expression::Literal(ref mut lit)) = value {
            match var_type {
                VarType::String => {
                    if lit.starts_with('\"') && lit.ends_with('\"') {
                        *lit = lit[1..lit.len() - 1].to_string();
                    }
                }
                VarType::Char { .. } => {
                    if lit.starts_with('\'') && lit.ends_with('\'') {
                        *lit = lit[1..lit.len() - 1].to_string();
                    }
                }
                _ => {}
            }
        }

        // Store variable in global map
        self.global_var_map
            .insert(name.clone(), (var_type.clone(), 0));

        Some(ASTNode::VarDeclaration {
            mutable,
            name,
            var_type,
            value,
        })
    }

    #[inline(always)]
    fn parse_print(&mut self, count: usize) -> Option<ASTNode> {
        let to_stderr = count == 3;
        self.pos += count; // consume '>' tokens
        self.skip_whitespace();
        
        // Check if the expression is a string literal
        if let Some(token) = self.current_token() {
            if token.kind == TokenKind::StringLiteral {
                let literal = self.get_lexeme(token).to_string();
                self.pos += 1; // Consume the string literal token
                self.consume_until_newline();
                
                return Some(ASTNode::Print { 
                    to_stderr, 
                    expr: Some(Expression::Literal(literal)) 
                });
            }
        }
        
        // Handle other expressions
        let expr = self.parse_expression_until_newline();
        self.consume_until_newline();
        Some(ASTNode::Print { to_stderr, expr })
    }

    #[inline(always)]
    fn parse_expression_until_newline(&mut self) -> Option<Expression> {
        self.skip_whitespace();
        if let Some(token) = self.current_token() {
            // Check if it's a string literal first
            if token.kind == TokenKind::StringLiteral {
                let literal = self.get_lexeme(token).to_string();
                self.pos += 1;
                return Some(Expression::Literal(literal));
            }
            
            // Otherwise handle other token types
            if token.kind == TokenKind::Identifier {
                let val = self.get_lexeme(token).to_string();
                self.pos += 1;
                
                // Try to determine if it's a number or identifier
                if val.chars().all(|c| c.is_digit(10) || c == '.') {
                    Some(Expression::Literal(val))
                } else {
                    Some(Expression::Identifier(val))
                }
            } else {
                self.pos += 1;
                None
            }
        } else {
            None
        }
    }

    #[inline(always)]
    fn consume_until_newline(&mut self) {
        while self.pos < self.tokens.len() && self.tokens[self.pos].kind != TokenKind::Newline {
            self.pos += 1;
        }
        self.pos += 1; // consume the newline token
    }
    #[inline(always)]
    pub fn get_global_pos(&self) -> usize {
        let mut global_pos = 0;
        for token in self.tokens.iter().take(self.pos) {
            global_pos += token.end - token.start;
        }
        if let Some(current_token) = self.tokens.get(self.pos) {
            global_pos += current_token.start;
        }
        global_pos + self.global_pos
    }

    #[inline(always)]
    pub fn get_line_number(&self, pos: usize) -> usize {
        // Count the number of newlines in the input up to the global position
        if pos >= self.tokens.len() {
            return 0;
        }
        
        let token = &self.tokens[pos];
        let mut line_count = 0;
        
        // Count newlines from start of input up to token position
        for i in 0..token.start {
            if i < self.input.len() && self.input.as_bytes()[i] == b'\n' {
                line_count += 1;
            }
        }
        
        // Line numbers are 1-based
        line_count + 1
    }
}
