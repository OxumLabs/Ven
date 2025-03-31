use std::collections::{HashMap, HashSet};
use std::slice;
use memchr::memchr;

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
                if let ASTNode::Print { to_stderr, expr } = &nodes[i] {
                    let mut exprs = Vec::new();
                    let initial_expr = expr.clone().unwrap_or(Expression::Literal(String::new()));
                    exprs.push(initial_expr);
                    
                    let mut j = i + 1;
                    while j < nodes.len() {
                        if let ASTNode::Print { to_stderr: t, expr: e } = &nodes[j] {
                            if t == to_stderr {
                                exprs.push(e.clone().unwrap_or(Expression::Literal(String::new())));
                                j += 1;
                                continue;
                            }
                        }
                        break;
                    }

                    let total_len: usize = exprs.iter().map(|e| expr_to_string(e).len()).sum();
                    let mut combined_str = String::with_capacity(total_len);
                    for e in exprs {
                        combined_str.push_str(&expr_to_string(&e));
                    }

                    if combined_str.starts_with(' ') {
                        combined_str = combined_str.split_off(1);
                    }

                    optimized.push(ASTNode::Print {
                        to_stderr: *to_stderr,
                        expr: Some(Expression::Literal(combined_str)),
                    });
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
            let bytes = lit.as_bytes();
            let len = bytes.len();
            let mut i = 0;
            
            unsafe {
                while i < len {
                    if *bytes.as_ptr().add(i) == b'{' {
                        if i > 0 && *bytes.as_ptr().add(i - 1) == b'\\' {
                            i += 1;
                            continue;
                        }
                        
                        if let Some(rel_close) = memchr(b'}', &bytes[i+1..]) {
                            let close = i + 1 + rel_close;
                            let mut var_start = i + 1;
                            while var_start < close && *bytes.as_ptr().add(var_start) == b' ' {
                                var_start += 1;
                            }
                            let mut var_end = close;
                            while var_end > var_start && *bytes.as_ptr().add(var_end - 1) == b' ' {
                                var_end -= 1;
                            }
                            if var_start < var_end {
                                let var_bytes = slice::from_raw_parts(bytes.as_ptr().add(var_start), var_end - var_start);
                                if let Ok(var_str) = std::str::from_utf8(var_bytes) {
                                    used.insert(var_str.to_string());
                                }
                            }
                            i = close + 1;
                            continue;
                        }
                    }
                    if i < len {
                        i += 1;
                    }
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