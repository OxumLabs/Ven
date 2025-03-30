use crate::parse::{ASTNode, Expression, MathOperator};
use crate::token::{Token, TokenKind};
use std::str;

/// Convert token to string slice efficiently (unsafe but fast)
#[inline(always)]
fn get_lexeme<'a>(token: &'a Token, input: &'a str) -> &'a str {
    unsafe { str::from_utf8_unchecked(&input.as_bytes()[token.start..token.end]) }
}

/// High-performance math expression parser
#[inline(always)]
pub fn parse1(tokens: &[Token], input: &str, _current_token: &Token) -> Option<ASTNode> {
    let mut pos = 0;
    let len = tokens.len();

    // Skip unknown tokens (whitespace, etc.)
    while pos < len && tokens[pos].kind == TokenKind::Unknown {
        pos += 1;
    }
    if pos >= len || tokens[pos].kind != TokenKind::Identifier {
        return None;
    }

    // Extract variable name
    let var_name = get_lexeme(&tokens[pos], input).to_owned();
    pos += 1;

    // Skip unknown tokens
    while pos < len && tokens[pos].kind == TokenKind::Unknown {
        pos += 1;
    }
    if pos >= len {
        return None;
    }

    // Identify operator
    let math_operator = match tokens[pos].kind {
        TokenKind::Plus => MathOperator::Add,
        TokenKind::Minus => MathOperator::Subtract,
        TokenKind::Star => MathOperator::Multiply,
        TokenKind::Slash => MathOperator::Divide,
        _ => return None,
    };
    pos += 1;

    // Skip unknown tokens
    while pos < len && tokens[pos].kind == TokenKind::Unknown {
        pos += 1;
    }

    // Extract operand efficiently
    let mut operand_str = String::with_capacity(32);
    while pos < len && tokens[pos].kind != TokenKind::Newline {
        if !operand_str.is_empty() {
            operand_str.push(' '); // Insert space only if needed
        }
        operand_str.push_str(get_lexeme(&tokens[pos], input));
        pos += 1;
    }

    Some(ASTNode::MathOp {
        name: var_name,
        operator: math_operator,
        operand: Expression::Literal(operand_str),
    })
}
