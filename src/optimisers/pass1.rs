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
                    if let ASTNode::Print { to_stderr, ref expr } = nodes[i] {
                        let mut combined = expr.clone().unwrap_or(Expression::Literal(String::new()));
                        let mut j = i + 1;
                        while j < nodes.len() {
                            if let ASTNode::Print { to_stderr: t, ref expr } = nodes[j] {
                                if t == to_stderr {
                                    let s1 = expr_to_string(&combined);
                                    let s2 = expr_to_string(expr.as_ref().unwrap_or(&Expression::Literal(String::new())));
                                    combined = Expression::Literal(format!("{} {}", s1, s2).trim().to_string());
                                    j += 1;
                                } else {
                                    break;
                                }
                            } else {
                                break;
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
            // Use memchr for faster scanning.
            use memchr::memchr;
            let bytes = lit.as_bytes();
            let mut i = 0;
            while i < bytes.len() {
                if bytes[i] == b'{' {
                    // Skip if escaped.
                    if i > 0 && bytes[i - 1] == b'\\' {
                        i += 1;
                        continue;
                    }
                    if let Some(close_rel) = memchr(b'}', &bytes[i + 1..]) {
                        let close = i + 1 + close_rel;
                        let var_name = &lit[i + 1..close];
                        let var_name = var_name.trim();
                        if !var_name.is_empty() {
                            used.insert(var_name.to_string());
                        }
                        i = close + 1;
                        continue;
                    } else {
                        break;
                    }
                }
                i += 1;
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
