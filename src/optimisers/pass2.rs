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
        Expression::Literal(lit) => Expression::Literal(replace_placeholders(&lit, inline_map)),
    }
}

#[inline(always)]
fn replace_placeholders(literal: &str, inline_map: &InlineMap) -> String {
    use memchr::memchr;
    let mut result = String::with_capacity(literal.len());
    let bytes = literal.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'{' {
            if i > 0 && bytes[i - 1] == b'\\' {
                result.push('{');
                i += 1;
            } else {
                let start = i + 1;
                if let Some(close_rel) = memchr(b'}', &bytes[start..]) {
                    let end = start + close_rel;
                    let placeholder = &literal[start..end].trim();
                    if !placeholder.is_empty() {
                        if let Some(replacement) = inline_map.get(*placeholder) {
                            result.push_str(replacement);
                        } else {
                            result.push('{');
                            result.push_str(placeholder);
                            result.push('}');
                        }
                    }
                    i = end + 1;
                } else {
                    result.push_str(&literal[i..]);
                    break;
                }
            }
        } else {
            // Fast copy using byte-wise push. Note: this may not handle multibyte characters correctly.
            result.push(literal.chars().nth(i).unwrap());
            i += 1;
        }
    }
    result
}
