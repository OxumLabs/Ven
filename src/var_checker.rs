use crate::errs::VarError;
use crate::parse::{AST, ASTNode, Expression, VarType};
use std::collections::HashMap;
use std::borrow::Cow;

pub type VarMap = HashMap<String, (VarType, usize)>;

#[inline(always)]
pub fn check_variables(ast: &AST, var_map: &mut VarMap) -> Vec<VarError> {
    let mut errors = Vec::new();
    let AST::Program(ref nodes) = *ast;

    for node in nodes {
        if let ASTNode::VarDeclaration { ref name, ref var_type, ref value, .. } = *node {
            let line = 0; // Placeholder for line number
            let var_type = var_type; // Copy instead of cloning
            var_map.insert(name.clone(), (var_type.clone(), line));

            if let Some(ref expr) = *value {
                let result = match (var_type, expr) {
                    (VarType::Int, Expression::Literal(lit)) => lit.parse::<i32>().is_err().then(|| VarError::VarTypeMisMatch {
                        var_name: name.clone(),
                        expected: "int".into(),
                        value: lit.clone(),
                        line,
                    }),

                    (VarType::Float, Expression::Literal(lit)) => lit.parse::<f64>().is_err().then(|| VarError::VarTypeMisMatch {
                        var_name: name.clone(),
                        expected: "float".into(),
                        value: lit.clone(),
                        line,
                    }),

                    (VarType::String, Expression::Literal(lit)) => (!lit.starts_with('\"') || !lit.ends_with('\"')).then(|| VarError::TypeMismatch {
                        var_name: name.clone(),
                        expected: "string literal enclosed in double quotes".into(),
                        found: lit.clone(),
                        line,
                    }),

                    (VarType::Char { size }, Expression::Literal(lit)) => {
                        if lit.starts_with('\'') && lit.ends_with('\'') {
                            let stripped = strip_char_quotes(lit);
                            (stripped.chars().count() != *size).then(|| VarError::TypeMismatch {
                                var_name: name.clone(),
                                expected: format!("char literal of size {}", size),
                                found: format!("char literal of size {}", stripped.chars().count()),
                                line,
                            })
                        } else {
                            Some(VarError::TypeMismatch {
                                var_name: name.clone(),
                                expected: "char literal enclosed in single quotes".into(),
                                found: lit.clone(),
                                line,
                            })
                        }
                    }

                    _ => Some(VarError::TypeMismatch {
                        var_name: name.clone(),
                        expected: format!("{:?}", var_type),
                        found: format!("{:?}", expr),
                        line,
                    }),
                };

                if let Some(err) = result {
                    errors.push(err);
                    var_map.insert(name.clone(), (VarType::Int, line));
                }
            }
        }
    }
    errors
}

#[inline(always)]
#[allow(dead_code)]
fn strip_quotes(s: &str) -> Cow<str> {
    if s.len() >= 2 && s.starts_with('\"') && s.ends_with('\"') {
        Cow::Borrowed(&s[1..s.len() - 1])
    } else {
        Cow::Borrowed(s)
    }
}

#[inline(always)]
fn strip_char_quotes(s: &str) -> Cow<str> {
    if s.len() >= 2 && s.starts_with('\'') && s.ends_with('\'') {
        Cow::Borrowed(&s[1..s.len() - 1])
    } else {
        Cow::Borrowed(s)
    }
}
