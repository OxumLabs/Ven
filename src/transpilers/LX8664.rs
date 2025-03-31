use crate::parse::{AST, ASTNode, Expression, MathOperator, VarType};
use std::collections::HashMap;
use std::fmt::Write;

fn escape_string(s: &str) -> String {
    let parts: Vec<String> = s
        .split('\n')
        .map(|p| p.replace("\"", "\\\""))
        .collect();
    if parts.len() > 1 {
        parts.join("\", 0x0A, \"")
    } else {
        parts[0].clone()
    }
}

pub fn transpile_lx8664(ast: &AST) -> String {
    // Preallocate buffers with a good capacity
    let mut data_section = String::with_capacity(512);
    let mut bss_section = String::with_capacity(256);
    let mut text_section = String::with_capacity(1024);
    let mut str_label_counter = 0;
    let mut var_label_counter = 0;

    let mut var_types: HashMap<String, VarType> = HashMap::new();
    let mut var_labels: HashMap<String, String> = HashMap::new();

    // Initialize Data Section with inline comment
    writeln!(&mut data_section, "section .data").unwrap();
    writeln!(&mut data_section, "newline: db 0x0A   ;; defines newline").unwrap();

    // Initialize BSS Section with inline comment
    writeln!(&mut bss_section, "section .bss").unwrap();
    writeln!(&mut bss_section, "    num_buffer: resb 20   ;; reserve 20 bytes for num_buffer").unwrap();

    // Initialize Text Section (Entry point)
    writeln!(&mut text_section, "section .text").unwrap();
    writeln!(&mut text_section, "global _start").unwrap();
    writeln!(&mut text_section, "_start:").unwrap();
    writeln!(&mut text_section, "    ;; _start: program entry point").unwrap();
    
    // First pass: allocate storage for variables
    match ast {
        AST::Program(nodes) => {
            for node in nodes {
                    if let ASTNode::VarDeclaration { name, var_type, mutable, .. } = node {
                        var_types.insert(name.clone(), var_type.clone());
                        let vlabel = format!("var_{}", var_label_counter);
                        var_label_counter += 1;
                        var_labels.insert(name.clone(), vlabel.clone());
                        match var_type {
                            VarType::String => {
                                if *mutable {
                                    writeln!(&mut bss_section, "    {}: resb 8192   ;; {} is mutable string, 8192 bytes", vlabel, name).unwrap();
                                } else {
                                    writeln!(&mut bss_section, "    {}: resb 256   ;; {} is immutable string, 256 bytes", vlabel, name).unwrap();
                                }
                            }
                            _ => {
                                writeln!(&mut bss_section, "    {}: resq 1   ;; {} is non-string, 8 bytes", vlabel, name).unwrap();
                            }
                        }
                    }
                }
            for node in nodes {
                    match node {
                        ASTNode::VarDeclaration { name, var_type, value, .. } => {
                            if let Some(Expression::Literal(val)) = value {
                                if let VarType::String = var_type {
                                    let label = format!("str_{}", str_label_counter);
                                    str_label_counter += 1;
                                    let replaced = val.replace("\\n", "\n");
                                    let processed = escape_string(&replaced);
                                    writeln!(&mut data_section, "    {}: db \"{}\", 0   ;; Literal for variable '{}' (label {})", label, processed, name, label).unwrap();
                                    let vlabel = var_labels.get(name).unwrap();
                                    // Inline the loading of the address with its comment
                                    writeln!(&mut text_section, "    lea rax, [{}]   ;; load address of {} into rax", label, label).unwrap();
                                    writeln!(&mut text_section, "    mov [{}], rax   ;; store address into variable '{}'", vlabel, name).unwrap();
                                } else {
                                    let vlabel = var_labels.get(name).unwrap();
                                    writeln!(&mut text_section, "    mov rax, {}   ;; set numeric variable '{}' to {}", val, name, val).unwrap();
                                    writeln!(&mut text_section, "    mov [{}], rax", vlabel).unwrap();
                                }
                            }
                        }
                        ASTNode::Input { name } => {
                            let vlabel = var_labels.get(name).unwrap();
                            writeln!(&mut text_section, "    mov rax, 0   ;; syscall: read").unwrap();
                            writeln!(&mut text_section, "    mov rdi, 0   ;; file descriptor: stdin").unwrap();
                            writeln!(&mut text_section, "    mov rsi, {}   ;; buffer for variable '{}'", vlabel, name).unwrap();
                            writeln!(&mut text_section, "    mov rdx, 8192   ;; buffer size").unwrap();
                            writeln!(&mut text_section, "    syscall").unwrap();
                            writeln!(&mut text_section, "    lea rdi, [{}]   ;; prepare address for trim_newline", vlabel).unwrap();
                            writeln!(&mut text_section, "    call trim_newline   ;; trim trailing newline").unwrap();
                        }
                        ASTNode::Print { to_stderr, expr } => {
                            let fd = if *to_stderr { "2" } else { "1" };
                            match expr {
                                Some(Expression::Literal(text)) => {
                                    #[derive(Debug)]
                                    enum PrintAction {
                                        Literal(String),
                                        Var(String),
                                    }
                                    let mut actions: Vec<PrintAction> = Vec::new();
                                    let mut buffer = String::new();
                                    let mut current_var = String::new();
                                    let mut in_var = false;
                                    for c in text.chars() {
                                        if c == '{' {
                                            if !buffer.is_empty() {
                                                actions.push(PrintAction::Literal(buffer.clone()));
                                                buffer.clear();
                                            }
                                            in_var = true;
                                            continue;
                                        } else if c == '}' {
                                            in_var = false;
                                            if !current_var.is_empty() {
                                                actions.push(PrintAction::Var(current_var.clone()));
                                                current_var.clear();
                                            }
                                            continue;
                                        }
                                        if in_var {
                                            current_var.push(c);
                                        } else {
                                            buffer.push(c);
                                        }
                                    }
                                    if !buffer.is_empty() {
                                        actions.push(PrintAction::Literal(buffer));
                                    }
                                    for action in actions {
                                        match action {
                                            PrintAction::Literal(lit) => {
                                                if !lit.is_empty() {
                                                    let label = format!("str_{}", str_label_counter);
                                                    str_label_counter += 1;
                                                    let (processed_str, add_newline) = if lit.ends_with("\\n") {
                                                        let mut s = lit.to_string();
                                                        s.truncate(s.len() - 2);
                                                        (s, true)
                                                    } else {
                                                        (lit.to_string(), false)
                                                    };
                                                    let processed = escape_string(&processed_str.replace("\\n", "\n"));
                                                    writeln!(&mut data_section, "    {}: db \"{}\", 0   ;; processed literal", label, processed).unwrap();
                                                    let text_len = processed.len() + 1;
                                                    writeln!(&mut text_section, "    mov rax, 1   ;; syscall: write literal").unwrap();
                                                    writeln!(&mut text_section, "    mov rdi, {}   ;; file descriptor {}", fd, fd).unwrap();
                                                    writeln!(&mut text_section, "    mov rsi, {}   ;; address of literal", label).unwrap();
                                                    writeln!(&mut text_section, "    mov rdx, {}   ;; length of literal", text_len).unwrap();
                                                    writeln!(&mut text_section, "    syscall").unwrap();
                                                    if add_newline {
                                                        writeln!(&mut text_section, "    mov rax, 1   ;; syscall: write newline").unwrap();
                                                        writeln!(&mut text_section, "    mov rdi, {}   ;; file descriptor {}", fd, fd).unwrap();
                                                        writeln!(&mut text_section, "    mov rsi, newline   ;; address of newline").unwrap();
                                                        writeln!(&mut text_section, "    mov rdx, 1   ;; length of newline").unwrap();
                                                        writeln!(&mut text_section, "    syscall").unwrap();
                                                    }
                                                }
                                            }
                                            PrintAction::Var(var) => {
                                                writeln!(&mut text_section, "    ;; Printing variable '{}'", var).unwrap();
                                                if let Some(var_type) = var_types.get(&var) {
                                                    if let VarType::String = var_type {
                                                        writeln!(&mut text_section, "    lea rsi, [{}]   ;; prepare address for printing string '{}'", var_labels.get(&var).unwrap(), var).unwrap();
                                                        writeln!(&mut text_section, "    call print_str").unwrap();
                                                    } else {
                                                        writeln!(&mut text_section, "    mov rax, [{}]   ;; load numeric variable '{}'", var_labels.get(&var).unwrap(), var).unwrap();
                                                        writeln!(&mut text_section, "    call print_int").unwrap();
                                                    }
                                                } else {
                                                    writeln!(&mut text_section, "    mov rax, [{}]   ;; load variable '{}'", var_labels.get(&var).unwrap(), var).unwrap();
                                                    writeln!(&mut text_section, "    call print_int").unwrap();
                                                }
                                            }
                                        }
                                    }
                                }
                                Some(Expression::Identifier(val)) => {
                                    if let Some(var_type) = var_types.get(val) {
                                        match var_type {
                                            VarType::String => {
                                                writeln!(&mut text_section, "    lea rsi, [{}]   ;; prepare string variable '{}' for printing", var_labels.get(val).unwrap(), val).unwrap();
                                                writeln!(&mut text_section, "    call print_str").unwrap();
                                            }
                                            _ => {
                                                writeln!(&mut text_section, "    mov rax, [{}]   ;; load numeric variable '{}'", var_labels.get(val).unwrap(), val).unwrap();
                                                writeln!(&mut text_section, "    call print_int").unwrap();
                                            }
                                        }
                                    } else {
                                        writeln!(&mut text_section, "    mov rax, [{}]   ;; load variable '{}'", var_labels.get(val).unwrap(), val).unwrap();
                                        writeln!(&mut text_section, "    call print_int").unwrap();
                                    }
                                }
                                None => {
                                    writeln!(&mut text_section, "    fprintf({}, \"\");   ;; print empty string", fd).unwrap();
                                }
                            }
                        }
                        ASTNode::MathOp { name, operator, operand } => {
                            writeln!(&mut text_section, "    ;; Processing math operation for '{}'", name).unwrap();
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
                            let vlabel = var_labels.get(name).unwrap();
                            if operator == &MathOperator::Divide {
                                writeln!(&mut text_section, "    mov rax, [{}]   ;; load variable '{}' for division", vlabel, name).unwrap();
                                writeln!(&mut text_section, "    cqo   ;; extend rax to rdx:rax").unwrap();
                                writeln!(&mut text_section, "    mov rcx, {}   ;; divisor", operand_str).unwrap();
                                writeln!(&mut text_section, "    idiv rcx   ;; perform division").unwrap();
                                writeln!(&mut text_section, "    mov [{}], rax   ;; store quotient back into '{}'", vlabel, name).unwrap();
                            } else {
                                writeln!(&mut text_section, "    mov rax, [{}]   ;; load variable '{}' for math op", vlabel, name).unwrap();
                                writeln!(&mut text_section, "    {} rax, {}   ;; perform math op", op, operand_str).unwrap();
                                writeln!(&mut text_section, "    mov [{}], rax   ;; store result into '{}'", vlabel, name).unwrap();
                            }
                        }
                    }
                }
        }
    }

    // Append the routines without modifying their structure but ensuring comments are inline where possible
    writeln!(&mut text_section, "\n    ;; Exiting program").unwrap();
    writeln!(&mut text_section, "    mov rax, 60   ;; syscall: exit").unwrap();
    writeln!(&mut text_section, "    xor rdi, rdi   ;; exit status 0").unwrap();
    writeln!(&mut text_section, "    syscall").unwrap();

    // print_int routine
    writeln!(&mut text_section, "\nprint_int:").unwrap();
    writeln!(&mut text_section, "    mov rsi, num_buffer + 19   ;; set pointer to end of num_buffer").unwrap();
    writeln!(&mut text_section, "    mov byte [rsi], 0   ;; null-terminate").unwrap();
    writeln!(&mut text_section, "    mov rbx, 10   ;; divisor for conversion").unwrap();
    writeln!(&mut text_section, ".int_to_str:").unwrap();
    writeln!(&mut text_section, "    dec rsi   ;; move pointer left").unwrap();
    writeln!(&mut text_section, "    xor rdx, rdx   ;; clear remainder register").unwrap();
    writeln!(&mut text_section, "    div rbx   ;; divide rax by 10").unwrap();
    writeln!(&mut text_section, "    add dl, '0'   ;; convert remainder to ASCII digit").unwrap();
    writeln!(&mut text_section, "    mov [rsi], dl   ;; store digit").unwrap();
    writeln!(&mut text_section, "    test rax, rax   ;; check if quotient is zero").unwrap();
    writeln!(&mut text_section, "    jnz .int_to_str   ;; loop if not zero").unwrap();
    writeln!(&mut text_section, "    mov rax, 1   ;; syscall: write").unwrap();
    writeln!(&mut text_section, "    mov rdi, 1   ;; file descriptor: stdout").unwrap();
    writeln!(&mut text_section, "    mov rdx, num_buffer + 19").unwrap();
    writeln!(&mut text_section, "    sub rdx, rsi   ;; compute length of converted string").unwrap();
    writeln!(&mut text_section, "    syscall   ;; write string").unwrap();
    writeln!(&mut text_section, "    ret").unwrap();

    // print_str routine
    writeln!(&mut text_section, "\nprint_str:").unwrap();
    writeln!(&mut text_section, "    mov rcx, 0   ;; initialize counter").unwrap();
    writeln!(&mut text_section, ".count_loop:").unwrap();
    writeln!(&mut text_section, "    cmp byte [rsi + rcx], 0   ;; check for null terminator").unwrap();
    writeln!(&mut text_section, "    je .check_newline").unwrap();
    writeln!(&mut text_section, "    inc rcx").unwrap();
    writeln!(&mut text_section, "    jmp .count_loop").unwrap();
    writeln!(&mut text_section, ".check_newline:").unwrap();
    writeln!(&mut text_section, "    cmp rcx, 0").unwrap();
    writeln!(&mut text_section, "    je .done").unwrap();
    writeln!(&mut text_section, "    dec rcx").unwrap();
    writeln!(&mut text_section, "    cmp byte [rsi + rcx], 0x0A   ;; check if last character is newline").unwrap();
    writeln!(&mut text_section, "    jne .print").unwrap();
    writeln!(&mut text_section, "    jmp .done").unwrap();
    writeln!(&mut text_section, ".print:").unwrap();
    writeln!(&mut text_section, "    inc rcx").unwrap();
    writeln!(&mut text_section, "    mov rax, 1").unwrap();
    writeln!(&mut text_section, "    mov rdi, 1").unwrap();
    writeln!(&mut text_section, "    mov rdx, rcx   ;; length of string").unwrap();
    writeln!(&mut text_section, "    syscall").unwrap();
    writeln!(&mut text_section, ".done:").unwrap();
    writeln!(&mut text_section, "    ret").unwrap();

    // trim_newline routine
    writeln!(&mut text_section, "\ntrim_newline:").unwrap();
    writeln!(&mut text_section, "    push rbx").unwrap();
    writeln!(&mut text_section, "    mov rcx, 0").unwrap();
    writeln!(&mut text_section, ".trim_loop:").unwrap();
    writeln!(&mut text_section, "    mov al, [rdi + rcx]").unwrap();
    writeln!(&mut text_section, "    cmp al, 0   ;; end of string check").unwrap();
    writeln!(&mut text_section, "    je .done").unwrap();
    writeln!(&mut text_section, "    cmp al, 0x0A   ;; check for newline").unwrap();
    writeln!(&mut text_section, "    je .replace").unwrap();
    writeln!(&mut text_section, "    inc rcx").unwrap();
    writeln!(&mut text_section, "    jmp .trim_loop").unwrap();
    writeln!(&mut text_section, ".replace:").unwrap();
    writeln!(&mut text_section, "    mov byte [rdi + rcx], 0   ;; replace newline with null").unwrap();
    writeln!(&mut text_section, ".done:").unwrap();
    writeln!(&mut text_section, "    pop rbx").unwrap();
    writeln!(&mut text_section, "    ret").unwrap();

    // Combine all sections
    let mut code = String::with_capacity(data_section.len() + bss_section.len() + text_section.len());
    code.push_str(&data_section);
    code.push_str("\n");
    code.push_str(&bss_section);
    code.push_str("\n");
    code.push_str(&text_section);
    code.shrink_to_fit();
    code
}
