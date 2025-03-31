use std::collections::{HashMap, HashSet};
use crate::parse::{AST, ASTNode, Expression, VarType};
pub type VarMap = HashMap<String, (VarType, usize)>;

#[inline(always)]
pub fn optimize_pass1(ast: &mut AST) {
    let used_vars = collect_used_vars(ast);
    match *ast {
        AST::Program(ref mut nodes) => {
            nodes.retain(|node| match node {
                    ASTNode::VarDeclaration { name, .. } => used_vars.contains(name),
                    _ => true,
                });
            let mut optimized = Vec::with_capacity(nodes.len());
            let mut i = 0;
            while i < nodes.len() {
                    // Combine adjacent Print nodes with the same `to_stderr` value.
                    if let ASTNode::Print { to_stderr, ref expr } = nodes[i] {
                        let mut combined = expr.clone().unwrap_or(Expression::Literal(String::new()));
                        let mut j = i + 1;
                        while j < nodes.len() {
                            if let ASTNode::Print { to_stderr: t, ref expr } = nodes[j] {
                                if t == to_stderr {
                                    let s1 = expr_to_string(&combined);
                                    let s2 = expr_to_string(expr.as_ref().unwrap_or(&Expression::Literal(String::new())));
                                    // Use format! to combine strings (this allocates, but it's simple and fast)
                                    combined = Expression::Literal(format!("{}{}", s1, s2));
                                    j += 1;
                                } else {
                                    break;
                                }
                            } else {
                                break;
                            }
                        }
                        // Remove exactly one leading space if present.
                        if let Expression::Literal(ref s) = combined {
                            if s.starts_with(' ') {
                                let new_str = s.replacen(" ", "", 1);
                                combined = Expression::Literal(new_str);
                            }
                        }
                        optimized.push(ASTNode::Print { to_stderr, expr: Some(combined) });
                        i = j;
                    } else {
                        optimized.push(nodes[i].clone());
                        i += 1;
                    }
                }
            *nodes = optimized;
        }
    }
}

#[inline(always)]
fn collect_used_vars(ast: &AST) -> HashSet<String> {
    let mut used = HashSet::with_capacity(64);
    match *ast {
        AST::Program(ref nodes) => {
            for node in nodes {
                    collect_used_vars_in_node(node, &mut used);
                }
        }
    }
    used
}

#[inline(always)]
fn collect_used_vars_in_node(node: &ASTNode, used: &mut HashSet<String>) {
    match node {
        ASTNode::Input { name } => { used.insert(name.clone()); },
        ASTNode::Print { expr, .. } => {
            if let Some(e) = expr { collect_used_vars_in_expression(e, used); }
        },
        ASTNode::VarDeclaration { value, .. } => {
            if let Some(e) = value { collect_used_vars_in_expression(e, used); }
        },
        ASTNode::MathOp { name, operand, .. } => {
            used.insert(name.clone());
            collect_used_vars_in_expression(operand, used);
        },
    }
}

#[inline(always)]
fn collect_used_vars_in_expression(expr: &Expression, used: &mut HashSet<String>) {
    match expr {
        Expression::Identifier(name) => { used.insert(name.clone()); },
        Expression::Literal(lit) => {
            // Optimized scanning using memchr and unsafe pointer arithmetic.
            use memchr::memchr;
            let bytes = lit.as_bytes();
            let len = bytes.len();
            let mut i = 0;
            unsafe {
                let ptr = bytes.as_ptr();
                while i < len {
                    if *ptr.add(i) == b'{' {
                        // Skip escaped '{'
                        if i > 0 && *ptr.add(i - 1) == b'\\' {
                            i += 1;
                            continue;
                        }
                        if let Some(rel) = memchr(b'}', &bytes[i + 1..]) {
                            let close = i + 1 + rel;
                            // SAFETY: We assume the substring is valid UTF-8.
                            let var_name = std::str::from_utf8_unchecked(&bytes[i + 1..close]);
                            let trimmed = var_name.trim();
                            if !trimmed.is_empty() {
                                used.insert(trimmed.to_string());
                            }
                            i = close + 1;
                            continue;
                        } else {
                            break;
                        }
                    }
                    i += 1;
                }
            }
        },
    }
}

#[inline(always)]
fn expr_to_string(expr: &Expression) -> String {
    match expr {
        Expression::Literal(s) => s.clone(),
        Expression::Identifier(s) => s.clone(),
    }
}
