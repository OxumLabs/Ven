use crate::parse::{AST, ASTNode, Expression, MathOperator, VarType};

pub fn transpile_rs(ast: &AST) -> String {
    let mut code = String::with_capacity(1024);

    // Add necessary imports at the top
    code.push_str("use std::io::Write;\n");
    code.push_str("fn main() {\n");

    match ast {
        AST::Program(nodes) => {
            for node in nodes {
                match node {
                    ASTNode::VarDeclaration {
                        mutable,
                        name,
                        var_type,
                        value,
                    } => {
                        let mutability = if *mutable { "mut " } else { "" };
                        let rust_type = match var_type {
                            VarType::Int => "i32",
                            VarType::String => "String",
                            VarType::Char { .. } => "char",
                            VarType::Float => "f64",
                        };
                        let value_str = match value {
                            Some(Expression::Literal(val)) => format!(" = {}", val),
                            Some(Expression::Identifier(val)) => format!(" = {}", val),
                            Some(Expression::BinaryOp { .. }) => String::new(),
                            Some(Expression::LogicalOp { .. }) => String::new(),
                            None => String::new(),
                        };
                        code.push_str(&format!(
                            "    let {}{}: {}{};\n",
                            mutability, name, rust_type, value_str
                        ));
                    }
                    ASTNode::Input { name } => {
                        code.push_str(&format!(
                            "    std::io::stdin().read_line(&mut {0}).unwrap();\n",
                            name
                        ));
                    }
                    ASTNode::Print { to_stderr, expr } => {
                        let output_target = if *to_stderr {
                            "std::io::stderr().lock()"
                        } else {
                            "std::io::stdout().lock()"
                        };

                        if let Some(Expression::Literal(val)) = expr {
                            code.push_str(&format!(
                                "    writeln!(&mut {}, \"{}\").unwrap();\n",
                                output_target, val
                            ));
                        } else if let Some(Expression::Identifier(val)) = expr {
                            code.push_str(&format!(
                                "    writeln!(&mut {}, \"{{}}\", {}).unwrap();\n",
                                output_target, val
                            ));
                        } else {
                            code.push_str(&format!(
                                "    writeln!(&mut {}).unwrap();\n",
                                output_target
                            ));
                        }
                    }
                    ASTNode::MathOp {
                        name,
                        operator,
                        operand,
                    } => {
                        let op = match operator {
                            MathOperator::Add => "+",
                            MathOperator::Subtract => "-",
                            MathOperator::Multiply => "*",
                            MathOperator::Divide => "/",
                        };
                        let operand_str = match operand {
                            Expression::Literal(val) => val.clone(),
                            Expression::Identifier(val) => val.clone(),
                            Expression::BinaryOp { left, operator, right } => {
                                let left_str = match **left {
                                    Expression::Literal(ref l) => l.clone(),
                                    Expression::Identifier(ref l) => l.clone(),
                                    _ => "".to_string(),
                                };
                                
                                let right_str = match **right {
                                    Expression::Literal(ref r) => r.clone(),
                                    Expression::Identifier(ref r) => r.clone(),
                                    _ => "".to_string(),
                                };
                                
                                let op_str = match operator {
                                    crate::parse::ComparisonOperator::Equal => "==",
                                    crate::parse::ComparisonOperator::NotEqual => "!=",
                                    crate::parse::ComparisonOperator::LessThan => "<",
                                    crate::parse::ComparisonOperator::LessThanEqual => "<=",
                                    crate::parse::ComparisonOperator::GreaterThan => ">",
                                    crate::parse::ComparisonOperator::GreaterThanEqual => ">=",
                                };
                                
                                format!("({} {} {})", left_str, op_str, right_str)
                            },
                            Expression::LogicalOp { left, operator, right } => {
                                let left_str = match **left {
                                    Expression::Literal(ref l) => l.clone(),
                                    Expression::Identifier(ref l) => l.clone(),
                                    _ => "".to_string(),
                                };
                                
                                let right_str = match **right {
                                    Expression::Literal(ref r) => r.clone(),
                                    Expression::Identifier(ref r) => r.clone(),
                                    _ => "".to_string(),
                                };
                                
                                let op_str = match operator {
                                    crate::parse::LogicalOperator::And => "&&",
                                    crate::parse::LogicalOperator::Or => "||",
                                };
                                
                                format!("({} {} {})", left_str, op_str, right_str)
                            },
                        };
                        code.push_str(&format!(
                            "    {} = {} {} {};\n",
                            name, name, op, operand_str
                        ));
                    }
                    _ => {}
                }
            }
        }
    }

    code.push_str("}\n");
    code.shrink_to_fit();
    code
}
