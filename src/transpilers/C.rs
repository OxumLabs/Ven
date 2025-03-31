use crate::parse::{AST, ASTNode, Expression, MathOperator, VarType};
use std::collections::HashMap;

pub fn transpile_c(ast: &AST) -> String {
    let mut code = String::with_capacity(1024);
    let mut temp_var_counter = 0; // Counter to ensure unique temp variables

    // Build a map from variable names to their declared types.
    let mut var_types: HashMap<String, VarType> = HashMap::new();
    match ast {
        AST::Program(nodes) => {
            for node in nodes {
                    if let ASTNode::VarDeclaration { name, var_type, .. } = node {
                        var_types.insert(name.clone(), var_type.clone());
                    }
                }
        }
    }

    // Add necessary C headers
    code.push_str("#include <stdio.h>\n");
    code.push_str("#include <stdlib.h>\n");
    // Added string.h for trimming newline from input.
    code.push_str("#include <string.h>\n");
    code.push_str("\nint main() {\n");

    match ast {
        AST::Program(nodes) => {
            for node in nodes {
                    match node {
                        // Variable Declarations
                        ASTNode::VarDeclaration {
                            mutable: _, // Mutability is ignored in C
                            name,
                            var_type,
                            value,
                        } => {
                            let c_type = match var_type {
                                VarType::Int => "int",
                                VarType::String => "char*",
                                VarType::Char { .. } => "char",
                                VarType::Float => "float",
                            };

                            let value_str = match value {
                                Some(Expression::Literal(val)) => format!(" = {}", val),
                                Some(Expression::Identifier(val)) => format!(" = {}", val),
                                None => String::new(),
                            };

                            // Allocate memory for strings
                            if let VarType::String = var_type {
                                code.push_str(&format!("    {} {} = malloc(256);\n", c_type, name));
                            } else {
                                code.push_str(&format!("    {} {}{};\n", c_type, name, value_str));
                            }
                        }

                        // Input Handling with newline trimming.
                        ASTNode::Input { name } => {
                            code.push_str(&format!(
                                "    fgets({}, 256, stdin);\n",
                                name
                            ));
                            // Trim newline if present.
                            code.push_str(&format!("    size_t len = strlen({});\n", name));
                            code.push_str(&format!("    if(len > 0 && {}[len-1] == '\\n') ", name));
                            code.push_str(&format!("{}[len-1] = '\\0';\n", name));
                        }

                        // Print Statements with type detection
                        ASTNode::Print { to_stderr, expr } => {
                            let output_target = if *to_stderr { "stderr" } else { "stdout" };

                            match expr {
                                Some(Expression::Literal(text)) => {
                                    let mut format_string = String::new();
                                    let mut var_references = Vec::new();
                                    let mut current_var = String::new();
                                    let mut in_var = false;

                                    // Process the literal to extract interpolation placeholders.
                                    for c in text.chars() {
                                        if c == '{' {
                                            in_var = true;
                                            continue;
                                        } else if c == '}' {
                                            in_var = false;
                                            if !current_var.is_empty() {
                                                // Determine the format specifier based on the variable type.
                                                let format_spec = match var_types.get(&current_var) {
                                                    Some(VarType::Int)   => "%d",
                                                    Some(VarType::Float) => "%f",
                                                    Some(VarType::String) => "%s",
                                                    Some(VarType::Char { .. }) => "%c",
                                                    None => "%d", // Default fallback.
                                                };
                                                format_string.push_str(format_spec);
                                                var_references.push(current_var.clone());
                                                current_var.clear();
                                            }
                                            continue;
                                        }

                                        if in_var {
                                            current_var.push(c);
                                        } else {
                                            format_string.push(c);
                                        }
                                    }

                                    // Build argument list for fprintf.
                                    let mut args = String::new();
                                    for var in var_references {
                                        args.push_str(&format!(", {}", var));
                                    }

                                    code.push_str(&format!(
                                        "    fprintf({}, \"{}\"{});\n",
                                        output_target, format_string, args
                                    ));
                                }
                                Some(Expression::Identifier(val)) => {
                                    // Look up the variable type.
                                    let format_spec = match var_types.get(val) {
                                        Some(VarType::Int) => "%d",
                                        Some(VarType::Float) => "%f",
                                        Some(VarType::String) => "%s",
                                        Some(VarType::Char { .. }) => "%c",
                                        None => "%s", // default fallback.
                                    };
                                    code.push_str(&format!(
                                        "    fprintf({}, \"{}\", {});\n",
                                        output_target, format_spec, val
                                    ));
                                }
                                None => {
                                    code.push_str(&format!("    fprintf({}, \"\");\n", output_target));
                                }
                            }
                        }

                        // Math Operations
                        ASTNode::MathOp { name, operator, operand } => {
                            let op = match operator {
                                MathOperator::Add => "+",
                                MathOperator::Subtract => "-",
                                MathOperator::Multiply => "*",
                                MathOperator::Divide => "/",
                            };

                            let operand_str = match operand {
                                Expression::Literal(val) => val.clone(),
                                Expression::Identifier(val) => {
                                    let unique_operand_var = format!("{}_{}", val, temp_var_counter);
                                    temp_var_counter += 1;
                                    code.push_str(&format!(
                                        "    int {} = {};\n",
                                        unique_operand_var, val
                                    ));
                                    unique_operand_var
                                }
                            };

                            let unique_new_var = format!("{}_{}", name, temp_var_counter);
                            temp_var_counter += 1;

                            // Perform operation
                            code.push_str(&format!(
                                "    int {} = {} {} {};\n",
                                unique_new_var, name, op, operand_str
                            ));

                            // Store the result
                            code.push_str(&format!(
                                "    {} = {};\n",
                                name, unique_new_var
                            ));
                        }
                    }
                }
        }
    }

    code.push_str("    return 0;\n}\n");
    code.shrink_to_fit();
    code
}
