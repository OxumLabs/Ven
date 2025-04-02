use crate::parse::{AST, ASTNode, Expression};
use std::collections::HashMap;

use super::pass1::optimize_pass1;
pub type InlineMap = HashMap<String, String>;

#[inline(always)]
pub fn pass2(ast: AST) -> AST {
    let inline_map = build_inline_map(&ast);
    match ast {
        AST::Program(nodes) => {
            let new_nodes = nodes
                .into_iter()
                .map(|node| inline_node(node, &inline_map))
                .collect();
            let mut ast = AST::Program(new_nodes);
            optimize_pass1(&mut ast);
            ast
        }
    }
}

#[inline(always)]
fn build_inline_map(ast: &AST) -> InlineMap {
    let mut map = HashMap::with_capacity(32);
    match ast {
        AST::Program(nodes) => {
            for node in nodes {
                if let ASTNode::VarDeclaration {
                    mutable,
                    name,
                    value,
                    ..
                } = node
                {
                    if !*mutable {
                        if let Some(Expression::Literal(lit)) = value {
                            map.insert(name.clone(), lit.clone());
                        }
                    }
                }
            }
        }
    }
    map
}

#[inline(always)]
fn inline_node(node: ASTNode, inline_map: &InlineMap) -> ASTNode {
    match node {
        ASTNode::If { condition, body } => {
            let new_condition = inline_expr(condition, inline_map);
            let new_body = body
                .into_iter()
                .map(|n| inline_node(n, inline_map))
                .collect();
            ASTNode::If {
                condition: new_condition,
                body: new_body,
            }
        }
        ASTNode::VarDeclaration {
            mutable,
            name,
            var_type,
            value,
        } => {
            let new_value = value.map(|expr| inline_expr(expr, inline_map));
            ASTNode::VarDeclaration {
                mutable,
                name,
                var_type,
                value: new_value,
            }
        }
        ASTNode::Input { name } => ASTNode::Input { name },
        ASTNode::Print { to_stderr, expr } => {
            let new_expr = expr.map(|e| inline_expr(e, inline_map));
            ASTNode::Print {
                to_stderr,
                expr: new_expr,
            }
        }
        ASTNode::MathOp {
            name,
            operator,
            operand,
        } => {
            let new_operand = inline_expr(operand, inline_map);
            ASTNode::MathOp {
                name,
                operator,
                operand: new_operand,
            }
        }
    }
}

#[inline(always)]
fn inline_expr(expr: Expression, inline_map: &InlineMap) -> Expression {
    match expr {
        Expression::Identifier(id) => inline_map
            .get(&id)
            .map(|s| Expression::Literal(s.clone()))
            .unwrap_or(Expression::Identifier(id)),
        Expression::Literal(lit) => {
            Expression::Literal(unsafe { replace_placeholders(lit.as_str(), inline_map) })
        }
        Expression::BinaryOp { left, operator, right } => {
            Expression::BinaryOp {
                left: Box::new(inline_expr(*left, inline_map)),
                operator,
                right: Box::new(inline_expr(*right, inline_map)),
            }
        }
        Expression::LogicalOp { left, operator, right } => {
            Expression::LogicalOp {
                left: Box::new(inline_expr(*left, inline_map)),
                operator,
                right: Box::new(inline_expr(*right, inline_map)),
            }
        }
    }
}

/// Ultra‑fast replacement of placeholders in a literal using unsafe pointer arithmetic.
/// This function is declared unsafe and further wraps every unsafe operation in explicit unsafe blocks.
#[inline(always)]
unsafe fn replace_placeholders(literal: &str, inline_map: &InlineMap) -> String {
    #[allow(unused_unsafe)]
    unsafe {
        let bytes = literal.as_bytes();
        let len = bytes.len();
        let mut result = String::with_capacity(len);
        let ptr = bytes.as_ptr();
        let mut i: usize = 0;
        while i < len {
            let b = unsafe { *ptr.add(i) };
            if b == b'{' {
                // Check for escaped '{'
                if i > 0 && unsafe { *ptr.add(i - 1) } == b'\\' {
                    result.push('{');
                    i += 1;
                    continue;
                }
                let start = i + 1;
                // Use memchr to locate the closing '}'
                if let Some(rel) = memchr::memchr(b'}', &bytes[start..]) {
                    let end = start + rel;
                    let placeholder = unsafe { std::str::from_utf8_unchecked(&bytes[start..end]) };
                    if !placeholder.is_empty() {
                        let key = placeholder.trim();
                        if let Some(replacement) = inline_map.get(key) {
                            result.push_str(replacement);
                        } else {
                            result.push('{');
                            result.push_str(placeholder);
                            result.push('}');
                        }
                    }
                    i = end + 1;
                    continue;
                } else {
                    result.push_str(unsafe { std::str::from_utf8_unchecked(&bytes[i..]) });
                    break;
                }
            } else {
                result.push(unsafe { *ptr.add(i) } as char);
                i += 1;
            }
        }
        // Remove exactly one leading space if it exists.
        if result.as_bytes().first() == Some(&b' ') {
            result = unsafe { result.get_unchecked(1..).to_string() };
        }
        result
    }
}
