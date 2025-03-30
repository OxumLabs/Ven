use crate::parse::{AST, ASTNode, Expression, MathOperator, VarType};

pub fn transpile_llvm(ast: &AST) -> String {
    //let mut code = String::new();
    let mut global_strings = String::new();
    let mut string_counter = 0;
    let mut temp_var_counter = 0; // Unique temp variable counter

    // Declare printf
    global_strings.push_str("declare i32 @printf(i8*, ...)\n");

    let mut main_code = String::new();
    main_code.push_str("define i32 @main() {\n");

    match ast {
        AST::Program(nodes) => {
            for node in nodes {
                match node {
                    ASTNode::VarDeclaration {
                        name,
                        var_type,
                        value,
                        ..
                    } => {
                        let llvm_type = match var_type {
                            VarType::Int => "i32",
                            VarType::Float => "double",
                            VarType::String => "i8*",
                            _ => "i32",
                        };

                        let value_str = match value {
                            Some(Expression::Literal(val)) => val.clone(),
                            Some(Expression::Identifier(val)) => format!("%{}", val),
                            None => "0".to_string(),
                        };

                        // Allocate variable
                        main_code.push_str(&format!("  %{} = alloca {}, align 4\n", name, llvm_type));

                        // Store initial value
                        if let Some(Expression::Literal(_)) = value {
                            main_code.push_str(&format!(
                                "  store {} {}, {}* %{}\n",
                                llvm_type, value_str, llvm_type, name
                            ));
                        }
                    }

                    ASTNode::Print { expr, .. } => {
                        if let Some(Expression::Literal(text)) = expr {
                            let mut format_string = String::new();
                            let mut var_references = Vec::new();
                            let mut current_var = String::new();
                            let mut in_var = false;

                            for c in text.chars() {
                                if c == '{' {
                                    in_var = true;
                                    continue;
                                } else if c == '}' {
                                    in_var = false;
                                    if !current_var.is_empty() {
                                        format_string.push_str("%d");
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

                            // Escape LLVM string
                            format_string.push('\0');
                            let escaped_str = format_string.replace("\n", "\\0A");

                            let format_length = escaped_str.len();
                            let format_label = format!("str{}", string_counter);
                            string_counter += 1;

                            // Add global string
                            global_strings.push_str(&format!(
                                "@{} = private unnamed_addr constant [{} x i8] c\"{}\", align 1\n",
                                format_label, format_length, escaped_str
                            ));

                            let mut llvm_args = String::new();
                            for var in var_references {
                                let unique_load_var = format!("{}_{}", var, temp_var_counter);
                                temp_var_counter += 1;

                                llvm_args.push_str(&format!(", i32 %{}", unique_load_var));

                                // Load variable
                                main_code.push_str(&format!(
                                    "  %{} = load i32, i32* %{}, align 4\n",
                                    unique_load_var, var
                                ));
                            }

                            // Call printf
                            main_code.push_str(&format!(
                                "  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([{} x i8], [{} x i8]* @{}, i32 0, i32 0) {})\n",
                                format_length, format_length, format_label, llvm_args
                            ));
                        }
                    }

                    ASTNode::MathOp {
                        name,
                        operator,
                        operand,
                    } => {
                        let op = match operator {
                            MathOperator::Add => "add",
                            MathOperator::Subtract => "sub",
                            MathOperator::Multiply => "mul",
                            MathOperator::Divide => "sdiv",
                        };

                        let operand_str = match operand {
                            Expression::Literal(val) => val.clone(),
                            Expression::Identifier(val) => {
                                let unique_operand_var = format!("{}_{}", val, temp_var_counter);
                                temp_var_counter += 1;

                                main_code.push_str(&format!(
                                    "  %{} = load i32, i32* %{}, align 4\n",
                                    unique_operand_var, val
                                ));
                                format!("%{}", unique_operand_var)
                            }
                        };

                        let unique_load_var = format!("{}_{}", name, temp_var_counter);
                        temp_var_counter += 1;
                        let unique_new_var = format!("{}_{}", name, temp_var_counter);
                        temp_var_counter += 1;

                        // Load the current value
                        main_code.push_str(&format!(
                            "  %{} = load i32, i32* %{}, align 4\n",
                            unique_load_var, name
                        ));

                        // Perform operation
                        main_code.push_str(&format!(
                            "  %{} = {} i32 %{}, {}\n",
                            unique_new_var, op, unique_load_var, operand_str
                        ));

                        // Store the result
                        main_code.push_str(&format!(
                            "  store i32 %{}, i32* %{}\n",
                            unique_new_var, name
                        ));
                    }

                    _ => {}
                }
            }
        }
    }

    main_code.push_str("  ret i32 0\n");
    main_code.push_str("}\n");

    // Ensure global strings come first
    format!("{}\n{}", global_strings, main_code)
}
