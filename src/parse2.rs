use crate::{
    errs::VarError,
    impl_parserstate::ParserState,
    parse::{ASTNode, Expression, ComparisonOperator, LogicalOperator},
    token::{Token, TokenKind}
};
use std::str;

#[inline(always)]
fn get_lexeme<'a>(token: &'a Token, input: &'a str) -> &'a str {
    // SAFETY: indices are assumed to be valid.
    unsafe { str::from_utf8_unchecked(&input.as_bytes()[token.start..token.end]) }
}

/// Parse a condition (logical expression)
#[inline(always)]
pub fn parse_condition(tokens: &[Token], start: usize, end: usize, input: &str, line: usize) -> Result<Expression, String> {
    if start >= end {
        return Err("Empty condition".to_string());
    }

    // First check for logical operators (higher precedence)
    for i in start..end {
        let token = &tokens[i];
        match token.kind {
            TokenKind::And => {
                let left = parse_condition(tokens, start, i, input, line)
                    .map_err(|e| format!("Invalid left side of AND: {}", e))?;
                let right = parse_condition(tokens, i + 1, end, input, line)
                    .map_err(|e| format!("Invalid right side of AND: {}", e))?;
                return Ok(Expression::LogicalOp {
                    left: Box::new(left),
                    operator: LogicalOperator::And,
                    right: Box::new(right),
                });
            }
            TokenKind::Or => {
                let left = parse_condition(tokens, start, i, input, line)
                    .map_err(|e| format!("Invalid left side of OR: {}", e))?;
                let right = parse_condition(tokens, i + 1, end, input, line)
                    .map_err(|e| format!("Invalid right side of OR: {}", e))?;
                return Ok(Expression::LogicalOp {
                    left: Box::new(left),
                    operator: LogicalOperator::Or,
                    right: Box::new(right),
                });
            }
            _ => {}
        }
    }

    // Check for comparison operators
    for i in start..end {
        let token = &tokens[i];
        // Handle all comparison operators, including Greater which is also used for print/input
        if matches!(token.kind, 
            TokenKind::Equal | 
            TokenKind::NotEqual | 
            TokenKind::LessThan | 
            TokenKind::LessEqual | 
            TokenKind::Greater | 
            TokenKind::GreaterEqual)
        {
            // Make sure to not go out of bounds
            if i > start && i < end - 1 {
                let left = parse_operand(tokens, start, i, input)
                    .ok_or_else(|| format!("Invalid left operand at position {}", i))?;
                let right = parse_operand(tokens, i + 1, end, input)
                    .ok_or_else(|| format!("Invalid right operand at position {}", i))?;
                
                let operator = match token.kind {
                    TokenKind::Equal => ComparisonOperator::Equal,
                    TokenKind::NotEqual => ComparisonOperator::NotEqual,
                    TokenKind::LessThan => ComparisonOperator::LessThan,
                    TokenKind::LessEqual => ComparisonOperator::LessThanEqual,
                    TokenKind::Greater => ComparisonOperator::GreaterThan,
                    TokenKind::GreaterEqual => ComparisonOperator::GreaterThanEqual,
                    _ => unreachable!(),
                };
                
                return Ok(Expression::BinaryOp {
                    left: Box::new(left),
                    operator,
                    right: Box::new(right),
                });
            } else {
                return Err(format!("Comparison operator at position {} is missing operands", i));
            }
        }
    }

    // If no operators found, it's a simple expression
    parse_operand(tokens, start, end, input)
        .ok_or_else(|| "Invalid condition expression".to_string())
}

/// Parse an operand (identifier or literal)
#[inline(always)]
fn parse_operand(tokens: &[Token], start: usize, end: usize, input: &str) -> Option<Expression> {
    if start >= end {
        return None;
    }

    // Handle identifiers and literals
    if end - start == 1 {
        let token = &tokens[start];
        let lexeme = get_lexeme(token, input);
        
        match token.kind {
            TokenKind::Identifier => {
                // Check if it's a number literal or an identifier
                if lexeme.chars().all(|c| c.is_digit(10) || c == '.') {
                    Some(Expression::Literal(lexeme.to_string()))
                } else {
                    Some(Expression::Identifier(lexeme.to_string()))
                }
            }
            TokenKind::StringLiteral => {
                // Handle string literals
                Some(Expression::Literal(lexeme.to_string()))
            }
            _ => Some(Expression::Literal(lexeme.to_string())),
        }
    } else {
        // For multiple tokens, try to find the actual identifier or literal
        // Skip whitespace and handle them properly
        let mut real_start = start;
        let mut real_end = end;
        
        // Skip leading whitespace
        while real_start < end {
            let token = &tokens[real_start];
            let lexeme = get_lexeme(token, input);
            if token.kind == TokenKind::Unknown && lexeme.trim().is_empty() {
                real_start += 1;
            } else {
                break;
            }
        }
        
        // Skip trailing whitespace
        while real_end > real_start {
            let token = &tokens[real_end - 1];
            let lexeme = get_lexeme(token, input);
            if token.kind == TokenKind::Unknown && lexeme.trim().is_empty() {
                real_end -= 1;
            } else {
                break;
            }
        }
        
        if real_start >= real_end {
            return None;
        }
        
        // If after trimming whitespace we have just one token, process it
        if real_end - real_start == 1 {
            let token = &tokens[real_start];
            let lexeme = get_lexeme(token, input);
            
            match token.kind {
                TokenKind::Identifier => {
                    // Check if it's a number literal or an identifier
                    if lexeme.chars().all(|c| c.is_digit(10) || c == '.') {
                        Some(Expression::Literal(lexeme.to_string()))
                    } else {
                        Some(Expression::Identifier(lexeme.to_string()))
                    }
                }
                TokenKind::StringLiteral => {
                    // Handle string literals
                    Some(Expression::Literal(lexeme.to_string()))
                }
                _ => Some(Expression::Literal(lexeme.to_string())),
            }
        } else {
            // Check if we have a string literal among the tokens
            for i in real_start..real_end {
                if tokens[i].kind == TokenKind::StringLiteral {
                    return Some(Expression::Literal(get_lexeme(&tokens[i], input).to_string()));
                }
            }
            
            // If we still have multiple tokens, join them as a literal
            let mut value = String::new();
            for i in real_start..real_end {
                value.push_str(get_lexeme(&tokens[i], input));
            }
            Some(Expression::Literal(value))
        }
    }
}

#[inline(always)]
pub fn parse2(
    tokens: &[Token],
    start_pos: usize,
    input: &str,
    parser_state: &mut ParserState,
) -> Option<ASTNode> {
    let mut pos = start_pos;
    let len = tokens.len();
    let line = parser_state.get_line_number(pos);

    println!("DEBUG PARSE2: Starting at position {}", pos);

    if pos >= len {
        println!("DEBUG PARSE2: Position out of bounds");
        return None;
    }

    // Check for '?' token - Question token
    if tokens[pos].kind != TokenKind::Question {
        println!("DEBUG PARSE2: Expected Question token, found {:?}", tokens[pos].kind);
        return None;
    }
    pos += 1;
    println!("DEBUG PARSE2: Found Question token, moving to position {}", pos);

    // Parse opening parenthesis
    if pos >= len || tokens[pos].kind != TokenKind::LSmallB {
        println!("DEBUG PARSE2: Missing open parenthesis at position {}", pos);
        parser_state
            .errors
            .push(VarError::MissingConditionOpenParen { line });
        return None;
    }
    pos += 1;
    println!("DEBUG PARSE2: Found open parenthesis, moving to position {}", pos);

    // Find the end of the condition (closing parenthesis)
    let condition_start = pos;
    let mut condition_end = pos;
    let mut paren_depth = 1;

    while condition_end < len {
        match tokens[condition_end].kind {
            TokenKind::LSmallB => paren_depth += 1,
            TokenKind::RSmallB => {
                paren_depth -= 1;
                if paren_depth == 0 {
                    break;
                }
            }
            TokenKind::Newline => {
                println!("DEBUG PARSE2: Unexpected newline in condition at position {}", condition_end);
                parser_state
                    .errors
                    .push(VarError::MissingConditionCloseParen { line });
                return None;
            }
            _ => {}
        }
        condition_end += 1;
    }

    if paren_depth != 0 || condition_end >= len {
        println!("DEBUG PARSE2: Missing close parenthesis, paren_depth={}", paren_depth);
        parser_state
            .errors
            .push(VarError::MissingConditionCloseParen { line });
        return None;
    }
    
    println!("DEBUG PARSE2: Found condition from position {} to {}", condition_start, condition_end);

    // Parse the condition expression
    let condition = match parse_condition(tokens, condition_start, condition_end, input, line) {
        Ok(expr) => expr,
        Err(details) => {
            println!("DEBUG PARSE2: Error parsing condition: {}", details);
            parser_state.errors.push(VarError::InvalidCondition { 
                details, 
                line
            });
            return None;
        }
    };
    
    println!("DEBUG PARSE2: Parsed condition: {:?}", condition);
    
    pos = condition_end + 1; // Move past the closing parenthesis
    println!("DEBUG PARSE2: Moving to position {} after condition", pos);

    // Skip any whitespace after the closing parenthesis
    while pos < len {
        let token = &tokens[pos];
        let lexeme = get_lexeme(token, input);
        if token.kind == TokenKind::Unknown && lexeme.trim().is_empty() {
            println!("DEBUG PARSE2: Skipping whitespace at position {}", pos);
            pos += 1;
        } else {
            break;
        }
    }
    println!("DEBUG PARSE2: After skipping whitespace, now at position {}", pos);

    // Parse opening brace for the body
    if pos >= len || tokens[pos].kind != TokenKind::LCurlyB {
        println!("DEBUG PARSE2: Missing open brace at position {}, found {:?}", pos, tokens[pos].kind);
        parser_state
            .errors
            .push(VarError::MissingBlockOpenBrace { line });
        return None;
    }
    pos += 1;
    println!("DEBUG PARSE2: Found open brace, moving to position {}", pos);

    // Parse the body of the conditional
    let mut body = Vec::new();
    let mut depth = 1;
    let body_start = pos;

    while pos < len {
        match tokens[pos].kind {
            TokenKind::LCurlyB => depth += 1,
            TokenKind::RCurlyB => {
                depth -= 1;
                if depth == 0 {
                    // Process the body tokens
                    let body_tokens = &tokens[body_start..pos];
                    println!("DEBUG PARSE2: Found body from position {} to {}", body_start, pos);

                    // Create a new parser state for the body
                    let mut state = ParserState {
                        tokens: body_tokens,
                        input,
                        global_pos: 0,
                        global_var_map: parser_state.global_var_map,
                        pos: 0,
                        errors: Vec::new(),
                    };

                    // Parse each statement in the body
                    while state.pos < body_tokens.len() {
                        if let Some(stmt) = state.parse_statement(body_tokens) {
                            println!("DEBUG PARSE2: Parsed body statement: {:?}", stmt);
                            body.push(stmt);
                        } else {
                            state.pos += 1;
                        }
                    }

                    // Append any errors from the inner parser
                    parser_state.errors.extend(state.errors);

                    // Update the outer parser position
                    parser_state.global_pos = pos + 1;
                    parser_state.pos = pos + 1;
                    
                    println!("DEBUG PARSE2: Successfully parsed conditional with {} body statements", body.len());

                    return Some(ASTNode::If { condition, body });
                }
            }
            _ => {}
        }
        pos += 1;
    }

    println!("DEBUG PARSE2: Unmatched closing brace");
    parser_state
        .errors
        .push(VarError::UnmatchedClosingBrace { line });
    None
}
