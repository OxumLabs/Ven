use crate::errs::VarError;
use crate::{
    parse1::parse1,
    token::{Token, TokenKind},
};
use std::collections::HashMap;

/// ------------------ AST Definitions ------------------

#[derive(Debug, Clone)]
pub enum VarType {
    Int,
    String,
    Char { size: usize },
    Float,
}

#[derive(Debug, Clone)]
pub enum Expression {
    Literal(String),
    Identifier(String),
}

/// Represents the available math operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MathOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

/// Extended AST node variants.
#[derive(Debug, Clone)]
pub enum ASTNode {
    /// Variable declaration.
    /// E.g.: "@ myVar i 42" (static) or "@@ myMutVar str hello" (mutable)
    VarDeclaration {
        mutable: bool,
        name: String,
        var_type: VarType,
        value: Option<Expression>,
    },
    /// Input statement.
    /// E.g.: "> myVar"
    Input { name: String },
    /// Print statement.
    /// E.g.: ">> Hello {name}!"
    Print {
        to_stderr: bool,
        expr: Option<Expression>,
    },
    /// Math operation on a mutable variable.
    /// E.g.: "* age + 2" or "* age / 3"
    MathOp {
        name: String,
        operator: MathOperator,
        operand: Expression,
    },
}

/// The top-level AST wrapping a program.
#[derive(Debug)]
pub enum AST {
    Program(Vec<ASTNode>),
}

impl AST {
    #[inline(always)]
    pub fn parse(
        tokens: &[Token],
        input: &str,
    ) -> (Self, HashMap<String, (VarType, usize)>, Vec<VarError>) {
        let mut nodes = Vec::with_capacity(tokens.len() / 4);
        let mut global_var_map = HashMap::with_capacity(16);
        let mut global_errors = Vec::with_capacity(8);
        let mut pos = 0;
        let total = tokens.len();

        while pos < total {
            let line_start = pos;
            while pos < total && tokens[pos].kind != TokenKind::Newline {
                pos += 1;
            }
            let line_tokens = &tokens[line_start..pos];
            if pos < total && tokens[pos].kind == TokenKind::Newline {
                pos += 1;
            }
            let mut state = ParserState {
                tokens: line_tokens,
                input,
                pos: 0,
                global_var_map: &mut global_var_map,
                errors: Vec::with_capacity(4),
            };
            while state.pos < state.tokens.len() {
                if let Some(stmt) = state.parse_statement() {
                    nodes.push(stmt);
                } else {
                    state.pos += 1;
                }
            }
            global_errors.extend(state.errors);
        }
        (AST::Program(nodes), global_var_map, global_errors)
    }

    #[inline(always)]
    pub fn debug(&self) {
        // Explicitly matching self as a reference.
        match self {
            &AST::Program(ref nodes) => {
                println!("AST Tree:");
                for (i, node) in nodes.iter().enumerate() {
                    let is_last = i == nodes.len() - 1;
                    Self::print_node(node, "", is_last);
                }
                println!("End of AST Tree");
            }
        }
    }

    #[inline(always)]
    fn print_node(node: &ASTNode, indent: &str, is_last: bool) {
        let branch = if is_last { "└── " } else { "├── " };
        match node {
            ASTNode::VarDeclaration {
                mutable,
                name,
                var_type,
                value,
            } => {
                println!("{}{}VarDeclaration", indent, branch);
                let child_indent = if is_last {
                    format!("{}    ", indent)
                } else {
                    format!("{}│   ", indent)
                };
                println!("{}├── Name: {}", child_indent, name);
                println!("{}├── Mutable: {}", child_indent, mutable);
                println!("{}└── Type: {:?}", child_indent, var_type);
                if let Some(expr) = value {
                    println!("{}    └── Value:", child_indent);
                    let expr_indent = format!("{}        ", child_indent);
                    Self::print_expression(expr, &expr_indent, true);
                }
            }
            ASTNode::Input { name } => {
                println!("{}{}Input", indent, branch);
                let child_indent = if is_last {
                    format!("{}    ", indent)
                } else {
                    format!("{}│   ", indent)
                };
                println!("{}    └── Name: {}", child_indent, name);
            }
            ASTNode::Print { to_stderr, expr } => {
                println!("{}{}Print", indent, branch);
                let child_indent = if is_last {
                    format!("{}    ", indent)
                } else {
                    format!("{}│   ", indent)
                };
                println!("{}├── to_stderr: {}", child_indent, to_stderr);
                if let Some(expr) = expr {
                    println!("{}└── Expression:", child_indent);
                    let expr_indent = format!("{}    ", child_indent);
                    Self::print_expression(expr, &expr_indent, true);
                }
            }
            ASTNode::MathOp {
                name,
                operator,
                operand,
            } => {
                println!("{}{}MathOp", indent, branch);
                let child_indent = if is_last {
                    format!("{}    ", indent)
                } else {
                    format!("{}│   ", indent)
                };
                println!("{}├── Name: {}", child_indent, name);
                println!("{}├── Operator: {:?}", child_indent, operator);
                println!("{}└── Operand:", child_indent);
                let expr_indent = format!("{}    ", child_indent);
                Self::print_expression(operand, &expr_indent, true);
            }
        }
    }
    #[inline(always)]
    fn print_expression(expr: &Expression, indent: &str, is_last: bool) {
        let branch = if is_last { "└── " } else { "├── " };
        match expr {
            Expression::Literal(lit) => println!("{}{}Literal: {}", indent, branch, lit),
            Expression::Identifier(id) => println!("{}{}Identifier: {}", indent, branch, id),
        }
    }
}

pub type VarMap = HashMap<String, (VarType, usize)>;

struct ParserState<'a> {
    tokens: &'a [Token],
    input: &'a str,
    pos: usize,
    global_var_map: &'a mut HashMap<String, (VarType, usize)>,
    errors: Vec<VarError>,
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
        while self.pos < self.tokens.len() {
            let token = &self.tokens[self.pos];
            if token.kind == TokenKind::Unknown && self.get_lexeme(token).trim().is_empty() {
                self.pos += 1;
            } else {
                break;
            }
        }
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
   fn parse_statement(&mut self) -> Option<ASTNode> {
       self.skip_whitespace();
       if let Some(token) = self.current_token() {
           let lexeme = self.get_lexeme(token);
           if lexeme == "*" {
               let token_clone = token.clone();
               self.pos += 1; // consume "*" token
               if let Some(math_node) = parse1(&self.tokens[self.pos..], self.input, &token_clone) {
                   self.consume_until_newline();
                   return Some(math_node);
               } else {
                   return None;
               }
           } else if lexeme == "@" {
               return self.parse_var_declaration();
           } else if lexeme == ">" {
               let count = self.count_consecutive_greater();
               if count == 1 {
                   return self.parse_input();
               } else if count == 2 || count == 3 {
                   return self.parse_print(count);
               } else {
                   self.pos += count;
                   return None;
               }
           }
           self.pos += 1;
           None
       } else {
           None
       }
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
       self.global_var_map.insert(name.clone(), (var_type.clone(), 0));

       Some(ASTNode::VarDeclaration {
           mutable,
           name,
           var_type,
           value,
       })
   }
   

    #[inline(always)]
    fn parse_input(&mut self) -> Option<ASTNode> {
        self.pos += 1; // consume '>'
        self.skip_whitespace();
        let name_token = self.current_token()?;
        if name_token.kind != TokenKind::Identifier {
            return None;
        }
        let name = self.get_lexeme(name_token).to_string();
        self.pos += 1;
        self.consume_until_newline();
        let line = 0;
        if !self.global_var_map.contains_key(&name) {
            self.errors.push(crate::errs::VarError::Undeclared {
                var_name: name.clone(),
                line,
            });
        }
        Some(ASTNode::Input { name })
    }

    #[inline(always)]
    fn parse_print(&mut self, count: usize) -> Option<ASTNode> {
        self.pos += count; // consume all '>' tokens
        self.skip_whitespace();
        let expr = self.parse_expression_until_newline();
        Some(ASTNode::Print {
            to_stderr: count == 3,
            expr,
        })
    }

    #[inline(always)]
    fn parse_expression_until_newline(&mut self) -> Option<Expression> {
        let start = self.pos;
        let mut parts = Vec::with_capacity(4);
        while self.pos < self.tokens.len() {
            let token = self.tokens[self.pos];
            let lex = self.get_lexeme(&token);
            if lex == "\n" {
                break;
            }
            // If token is Unknown and its lexeme is "{" or "}", skip it.
            if token.kind == TokenKind::Unknown && (lex == "{" || lex == "}") {
                self.pos += 1;
                continue;
            }
            parts.push(lex.to_string());
            self.pos += 1;
        }
        if self.pos == start {
            return None;
        }
        let literal = parts.join("").to_string();
        self.check_placeholders(&literal);
        Some(Expression::Literal(literal))
    }

    #[inline(always)]
    fn check_placeholders(&mut self, literal: &str) {
        let bytes = literal.as_bytes();
        let len = bytes.len();
        let mut i = 0;
        while i < len {
            if bytes[i] == b'{' {
                if i > 0 && bytes[i - 1] == b'\\' {
                    i += 1;
                    continue;
                }
                let mut j = i + 1;
                while j < len && bytes[j] != b'}' {
                    j += 1;
                }
                if j < len {
                    let placeholder = &literal[i + 1..j].trim();
                    if !placeholder.is_empty()
                        && !self.global_var_map.contains_key(&placeholder.to_string())
                    {
                        self.errors.push(crate::errs::VarError::Undeclared {
                            var_name: placeholder.to_string(),
                            line: 0,
                        });
                    }
                    i = j + 1;
                    continue;
                } else {
                    break;
                }
            } else {
                i += 1;
            }
        }
    }

    #[inline(always)]
    fn consume_until_newline(&mut self) {
        while self.pos < self.tokens.len() {
            let token = self.tokens[self.pos];
            if self.get_lexeme(&token) == "\n" {
                self.pos += 1;
                break;
            }
            self.pos += 1;
        }
    }
}
