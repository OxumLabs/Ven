use crate::parse::{AST, ASTNode, Expression, MathOperator, VarType};

pub fn transpile_lx8664(ast: &AST) -> String {
    let mut code = String::with_capacity(1024);
    let mut variables = vec![];
    let mut data_section = String::new();
    let mut bss_section = String::new();
    let mut text_section = String::new();
    let mut var_counter = 0;

    data_section.push_str("section .data\n");
    bss_section.push_str("section .bss\n    num_buffer: resb 20\n");
    text_section.push_str("section .text\nglobal _start\n_start:\n");

    match ast {
        AST::Program(nodes) => {
            for node in nodes {
                match node {
                    ASTNode::VarDeclaration {
                        mutable: _,
                        name,
                        var_type,
                        value,
                    } => {
                        variables.push(name.clone());
                        if let VarType::String = var_type {
                            bss_section.push_str(&format!("    {}: resb 256\n", name));
                        } else {
                            bss_section.push_str(&format!("    {}: resq 1\n", name));
                        }

                        if let Some(Expression::Literal(val)) = value {
                            text_section.push_str(&format!("    mov rax, {}\n    mov [{}], rax\n", val, name));
                        }
                    }

                    ASTNode::Input { name } => {
                        text_section.push_str(&format!(
                            "    mov rax, 0\n    mov rdi, 0\n    mov rsi, {}\n    mov rdx, 256\n    syscall\n",
                            name
                        ));
                    }

                    ASTNode::Print { to_stderr, expr } => {
                        let fd = if *to_stderr { "2" } else { "1" };

                        match expr {
                            Some(Expression::Literal(text)) => {
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

                                let formatted_text = format_string.replace("\n", "0x0A");
                                let label = format!("str_{}", var_counter);
                                var_counter += 1;
                                data_section.push_str(&format!("    {}: db {:?}, 0x0A, 0\n", label, formatted_text));

                                text_section.push_str(&format!(
                                    "    mov rax, 1\n    mov rdi, {}\n    mov rsi, {}\n    mov rdx, {}\n    syscall\n",
                                    fd, label, formatted_text.len() + 1
                                ));

                                for var in var_references {
                                    text_section.push_str(&format!(
                                        "    mov rax, [{}]\n    call print_int\n",
                                        var
                                    ));
                                }
                            }
                            Some(Expression::Identifier(val)) => {
                                text_section.push_str(&format!(
                                    "    mov rax, [{}]\n    call print_int\n",
                                    val
                                ));
                            }
                            None => {
                                text_section.push_str("    ; Empty print\n");
                            }
                        }
                    }

                    ASTNode::MathOp { name, operator, operand } => {
                        let op = match operator {
                            MathOperator::Add => "add",
                            MathOperator::Subtract => "sub",
                            MathOperator::Multiply => "imul",
                            MathOperator::Divide => "idiv",
                        };

                        let operand_str = match operand {
                            Expression::Literal(val) => val.clone(),
                            Expression::Identifier(val) => val.clone(),
                        };

                        if operator == &MathOperator::Divide {
                            text_section.push_str(&format!(
                                "    mov rax, [{}]\n    cqo\n    mov rcx, {}\n    idiv rcx\n    mov [{}], rax\n",
                                name, operand_str, name
                            ));
                        } else {
                            text_section.push_str(&format!(
                                "    mov rax, [{}]\n    {} rax, {}\n    mov [{}], rax\n",
                                name, op, operand_str, name
                            ));
                        }
                    }
                }
            }
        }
    }

    text_section.push_str("\n    mov rax, 60\n    xor rdi, rdi\n    syscall\n");

    text_section.push_str(
        r#"
print_int:
    mov rsi, num_buffer + 19
    mov byte [rsi], 0
    mov rbx, 10

.int_to_str:
    dec rsi
    xor rdx, rdx
    div rbx
    add dl, '0'
    mov [rsi], dl
    test rax, rax
    jnz .int_to_str

    mov rax, 1
    mov rdi, 1
    mov rdx, num_buffer + 19
    sub rdx, rsi
    syscall
    ret
"#,
    );

    code.push_str(&data_section);
    code.push_str("\n");
    code.push_str(&bss_section);
    code.push_str("\n");
    code.push_str(&text_section);
    code.shrink_to_fit();
    code
}
