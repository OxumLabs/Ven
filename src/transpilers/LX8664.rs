use crate::parse::{AST, ASTNode, Expression, VarType};
use std::collections::HashMap;
use std::fmt::Write;

// Add a helper function to properly escape strings for assembly
fn escape_string_for_assembly(input: &str) -> String {
    // For NASM, we can use the simpler approach of removing any quotes that might cause issues
    input.replace("\"", "")
}

pub fn transpile_lx8664(ast: &AST) -> String {
    // Initialize sections for assembly code
    let mut text_section = String::new();
    let mut data_section = String::new();
    let mut bss_section = String::new();
    
    // Variable maps for tracking declarations
    let mut var_labels = HashMap::new();
    let mut var_types = HashMap::new();
    
    let mut str_label_counter = 0;
    
    // Initialize Data Section
    writeln!(&mut data_section, "section .data").unwrap();
    writeln!(&mut data_section, "newline: db 0x0A   ;; defines newline").unwrap();
    
    // Add buffer for integer conversion
    data_section.push_str("    int_buffer: times 16 db 0  ;; Buffer for integer/float string conversion\n");
    
    // Initialize BSS Section with inline comment
    bss_section.push_str("section .bss\n");
    bss_section.push_str("    num_buffer: resb 20   ;; reserve 20 bytes for num_buffer\n");
    
    // First pass: process variable declarations to setup data section
    let mut var_counter = 0;
    
    let AST::Program(nodes) = ast;
    
    for node in nodes {
        if let ASTNode::VarDeclaration { name, var_type, mutable: _, value: _ } = node {
            let vlabel = format!("var_{}", var_counter);
            var_counter += 1;
            
            match var_type {
                VarType::Int | VarType::Float => {
                    bss_section.push_str(&format!("    {}: resq 1   ;; {} is non-string, 8 bytes\n", vlabel, name));
                },
                VarType::String => {
                    bss_section.push_str(&format!("    {}: resb 256   ;; {} is string, reserve 256 bytes\n", vlabel, name));
                },
                VarType::Char { .. } => {
                    bss_section.push_str(&format!("    {}: resb 8   ;; {} is char, reserve 8 bytes\n", vlabel, name));
                }
            }
            
            var_labels.insert(name.clone(), vlabel);
            var_types.insert(name.clone(), var_type.clone());
        }
    }
    
    // Initialize Text Section with entry point
    writeln!(&mut text_section, "section .text").unwrap();
    writeln!(&mut text_section, "global _start").unwrap();
    writeln!(&mut text_section, "_start:").unwrap();
    writeln!(&mut text_section, "    ;; _start: program entry point").unwrap();
    
    // Process all instructions, generating code in the text_section
    for (_i, node) in nodes.iter().enumerate() {
        match node {
            ASTNode::Print { expr, to_stderr } => {
                let fd = if *to_stderr { "2" } else { "1" };
                
                match expr {
                    Some(Expression::Literal(text)) => {
                        let clean_str = text.trim_matches('"');
                        
                        // Check for variable interpolation (patterns like {variable})
                        // We need to handle interpolation at runtime, not compile time
                        if clean_str.contains('{') {
                            // String has interpolation - parse and process at runtime
                            let mut current_pos = 0;
                            let mut segments = Vec::new();
                            
                            // Split the string into text segments and variable references
                            while current_pos < clean_str.len() {
                                if let Some(start_idx) = clean_str[current_pos..].find('{') {
                                    let abs_start = current_pos + start_idx;
                                    
                                    // Add the text before the variable reference
                                    if abs_start > current_pos {
                                        let text_segment = &clean_str[current_pos..abs_start];
                                        if !text_segment.is_empty() {
                                            segments.push(("text", text_segment.to_string()));
                                        }
                                    }
                                    
                                    // Find the closing brace
                                    if let Some(end_idx) = clean_str[abs_start+1..].find('}') {
                                        let abs_end = abs_start + 1 + end_idx;
                                        let var_name = &clean_str[abs_start+1..abs_end];
                                        
                                        // Add the variable reference
                                        segments.push(("var", var_name.to_string()));
                                        current_pos = abs_end + 1;
                                    } else {
                                        // No closing brace found - treat as regular text
                                        let text_segment = &clean_str[current_pos..];
                                        segments.push(("text", text_segment.to_string()));
                                        break;
                                    }
                                } else {
                                    // No more variable references - add the rest as text
                                    let text_segment = &clean_str[current_pos..];
                                    if !text_segment.is_empty() {
                                        segments.push(("text", text_segment.to_string()));
                                    }
                                    break;
                                }
                            }
                            
                            // Now generate code for each segment
                            for (_i, (segment_type, segment_content)) in segments.iter().enumerate() {
                                if segment_type == &"text" {
                                    if !segment_content.is_empty() {
                                        let label = format!("str_{}", str_label_counter);
                                        str_label_counter += 1;
                                        let escaped_str = escape_string_for_assembly(segment_content);
                                        
                                        // Add to data section
                                        writeln!(&mut data_section, "    {}: db \"{}\", 0", label, escaped_str).unwrap();
                                        
                                        // Print the text segment
                                        writeln!(&mut text_section, "    ;; Print text segment \"{}\"", escaped_str).unwrap();
                                        writeln!(&mut text_section, "    mov rax, 1   ;; syscall: write").unwrap();
                                        writeln!(&mut text_section, "    mov rdi, {}   ;; file descriptor", fd).unwrap();
                                        writeln!(&mut text_section, "    mov rsi, {}   ;; address of string", label).unwrap();
                                        writeln!(&mut text_section, "    mov rdx, {}   ;; length of string", escaped_str.len()).unwrap();
                                        writeln!(&mut text_section, "    syscall").unwrap();
                                    }
                                } else if segment_type == &"var" {
                                    // Handle variable reference - THIS IS CRUCIAL FOR RUNTIME INTERPOLATION
                                    let var_name = segment_content.as_str();
                                    if var_types.contains_key(var_name) {
                                        writeln!(&mut text_section, "    ;; Print runtime variable value '{}'", var_name).unwrap();
                                        
                                        let var_type = var_types.get(var_name).unwrap();
                                        match var_type {
                                            VarType::String => {
                                                // For string variables in BSS
                                                writeln!(&mut text_section, "    lea rsi, [{}]   ;; load address of string variable", var_labels.get(var_name).unwrap()).unwrap();
                                                writeln!(&mut text_section, "    call print_str").unwrap();
                                            }
                                            VarType::Int => {
                                                // For integer variables
                                                writeln!(&mut text_section, "    mov rax, [{}]   ;; load integer variable at runtime", var_labels.get(var_name).unwrap()).unwrap();
                                                writeln!(&mut text_section, "    call print_int").unwrap();
                                            }
                                            VarType::Float => {
                                                // For float variables
                                                writeln!(&mut text_section, "    mov rax, [{}]   ;; load float variable at runtime", var_labels.get(var_name).unwrap()).unwrap();
                                                writeln!(&mut text_section, "    call print_int   ;; FIXME: should use print_float").unwrap();
                                            }
                                            VarType::Char { .. } => {
                                                // For char variables
                                                writeln!(&mut text_section, "    lea rsi, [{}]   ;; load address of char variable", var_labels.get(var_name).unwrap()).unwrap();
                                                writeln!(&mut text_section, "    call print_str").unwrap();
                                            }
                                        }
                                    } else {
                                        // Variable not found - print error placeholder
                                        let label = format!("str_err_{}", str_label_counter);
                                        str_label_counter += 1;
                                        
                                        writeln!(&mut data_section, "    {}: db \"[undefined]\", 0", label).unwrap();
                                        writeln!(&mut text_section, "    ;; Print error for undefined variable '{}'", var_name).unwrap();
                                        writeln!(&mut text_section, "    mov rax, 1   ;; syscall: write").unwrap();
                                        writeln!(&mut text_section, "    mov rdi, {}   ;; file descriptor", fd).unwrap();
                                        writeln!(&mut text_section, "    mov rsi, {}   ;; address of string", label).unwrap();
                                        writeln!(&mut text_section, "    mov rdx, 11   ;; length of [undefined]").unwrap();
                                        writeln!(&mut text_section, "    syscall").unwrap();
                                    }
                                }
                            }
                            
                            // Add newline at the end of the entire interpolated string
                            writeln!(&mut text_section, "    mov rax, 1   ;; syscall: write newline").unwrap();
                            writeln!(&mut text_section, "    mov rdi, {}   ;; file descriptor", fd).unwrap();
                            writeln!(&mut text_section, "    mov rsi, newline   ;; address of newline").unwrap();
                            writeln!(&mut text_section, "    mov rdx, 1   ;; length of newline").unwrap();
                            writeln!(&mut text_section, "    syscall").unwrap();
                        } else {
                            // No interpolation - just print the literal string
                            let label = format!("str_{}", str_label_counter);
                            str_label_counter += 1;
                            let escaped_str = escape_string_for_assembly(clean_str);
                            
                            // Add to data section
                            writeln!(&mut data_section, "    {}: db \"{}\", 0", label, escaped_str).unwrap();
                            
                            // Print the string
                            writeln!(&mut text_section, "    ;; Print string literal: \"{}\"", escaped_str).unwrap();
                            writeln!(&mut text_section, "    mov rax, 1   ;; syscall: write").unwrap();
                            writeln!(&mut text_section, "    mov rdi, {}   ;; file descriptor", fd).unwrap();
                            writeln!(&mut text_section, "    mov rsi, {}   ;; address of string", label).unwrap();
                            writeln!(&mut text_section, "    mov rdx, {}   ;; length of string", escaped_str.len()).unwrap();
                            writeln!(&mut text_section, "    syscall").unwrap();
                            
                            // Print newline
                            writeln!(&mut text_section, "    mov rax, 1   ;; syscall: write newline").unwrap();
                            writeln!(&mut text_section, "    mov rdi, {}   ;; file descriptor", fd).unwrap();
                            writeln!(&mut text_section, "    mov rsi, newline   ;; address of newline").unwrap();
                            writeln!(&mut text_section, "    mov rdx, 1   ;; length of newline").unwrap();
                            writeln!(&mut text_section, "    syscall").unwrap();
                        }
                    },
                    Some(Expression::Identifier(var_name)) => {
                        if let Some(var_type) = var_types.get(var_name) {
                            match var_type {
                                VarType::String => {
                                    writeln!(&mut text_section, "    ;; Printing string variable '{}'", var_name).unwrap();
                                    writeln!(&mut text_section, "    lea rsi, [{}]   ;; prepare string variable '{}' for printing", var_labels.get(var_name).unwrap(), var_name).unwrap();
                                    writeln!(&mut text_section, "    call print_str").unwrap();
                                    // Add newline after printing
                                    writeln!(&mut text_section, "    mov rax, 1   ;; syscall: write newline").unwrap();
                                    writeln!(&mut text_section, "    mov rdi, 1   ;; file descriptor 1 (stdout)").unwrap();
                                    writeln!(&mut text_section, "    mov rsi, newline   ;; address of newline").unwrap();
                                    writeln!(&mut text_section, "    mov rdx, 1   ;; length of newline").unwrap();
                                    writeln!(&mut text_section, "    syscall").unwrap();
                                }
                                _ => {
                                    writeln!(&mut text_section, "    ;; Printing numeric variable '{}'", var_name).unwrap();
                                    writeln!(&mut text_section, "    mov rax, [{}]   ;; load numeric variable '{}'", var_labels.get(var_name).unwrap(), var_name).unwrap();
                                    writeln!(&mut text_section, "    call print_int").unwrap();
                                    // Add newline after printing
                                    writeln!(&mut text_section, "    mov rax, 1   ;; syscall: write newline").unwrap();
                                    writeln!(&mut text_section, "    mov rdi, 1   ;; file descriptor 1 (stdout)").unwrap();
                                    writeln!(&mut text_section, "    mov rsi, newline   ;; address of newline").unwrap();
                                    writeln!(&mut text_section, "    mov rdx, 1   ;; length of newline").unwrap();
                                    writeln!(&mut text_section, "    syscall").unwrap();
                                }
                            }
                        } else {
                            // Default to integer if type is unknown
                            writeln!(&mut text_section, "    ;; Printing variable '{}' with unknown type", var_name).unwrap();
                            writeln!(&mut text_section, "    mov rax, [{}]   ;; load variable '{}'", var_labels.get(var_name).unwrap(), var_name).unwrap();
                            writeln!(&mut text_section, "    call print_int").unwrap();
                            // Add newline after printing
                            writeln!(&mut text_section, "    mov rax, 1   ;; syscall: write newline").unwrap();
                            writeln!(&mut text_section, "    mov rdi, 1   ;; file descriptor 1 (stdout)").unwrap();
                            writeln!(&mut text_section, "    mov rsi, newline   ;; address of newline").unwrap();
                            writeln!(&mut text_section, "    mov rdx, 1   ;; length of newline").unwrap();
                            writeln!(&mut text_section, "    syscall").unwrap();
                        }
                    },
                    Some(Expression::BinaryOp { .. }) | Some(Expression::LogicalOp { .. }) => {
                        // Skip complex expressions during string processing
                        // These will be handled in the expression evaluation phase
                        writeln!(&mut text_section, "    ;; Complex expression in print statement").unwrap();
                        writeln!(&mut text_section, "    ;; TODO: Implement printing of complex expressions").unwrap();
                    },
                    None => {
                        writeln!(&mut text_section, "    fprintf({}, \"\");   ;; print empty string", fd).unwrap();
                    }
                }
            },
            ASTNode::VarDeclaration { name, var_type: _, mutable: _, value } => {
                // Variable declarations already processed in first pass
                writeln!(&mut text_section, "    ;; Variable '{}' already declared in data/bss section", name).unwrap();
                
                // For variable initialization, add code to set initial value
                if let Some(Expression::Literal(val)) = value {
                    writeln!(&mut text_section, "    ;; Initialize variable '{}'", name).unwrap();
                    let operand_str = match val.as_str() {
                        "true" => "1",
                        "false" => "0",
                        _ => val,
                    };
                    writeln!(&mut text_section, "    mov qword [{}], {}   ;; set value of '{}'", var_labels.get(name).unwrap(), operand_str, name).unwrap();
                }
            },
            ASTNode::MathOp { name, operator, operand } => {
                writeln!(&mut text_section, "    ;; Math operation on variable '{}'", name).unwrap();
                
                // Load the variable's current value into RAX
                writeln!(&mut text_section, "    mov rax, [{}]   ;; load variable", var_labels.get(name).unwrap()).unwrap();
                
                match operand {
                    Expression::Literal(val) => {
                        // Handle literal operand
                        let operand_val = val.parse::<i64>().unwrap_or(0);
                        match operator {
                            crate::parse::MathOperator::Add => {
                                writeln!(&mut text_section, "    add rax, {}   ;; add literal value", operand_val).unwrap();
                            },
                            crate::parse::MathOperator::Subtract => {
                                writeln!(&mut text_section, "    sub rax, {}   ;; subtract literal value", operand_val).unwrap();
                            },
                            crate::parse::MathOperator::Multiply => {
                                writeln!(&mut text_section, "    imul rax, {}   ;; multiply by literal value", operand_val).unwrap();
                            },
                            crate::parse::MathOperator::Divide => {
                                writeln!(&mut text_section, "    mov rcx, {}   ;; set divisor", operand_val).unwrap();
                                writeln!(&mut text_section, "    cqo   ;; sign-extend RAX to RDX:RAX for division").unwrap();
                                writeln!(&mut text_section, "    idiv rcx   ;; divide RDX:RAX by RCX").unwrap();
                            },
                        }
                    },
                    Expression::Identifier(var_name) => {
                        // Handle variable operand
                        if var_labels.contains_key(var_name) {
                            writeln!(&mut text_section, "    mov rcx, [{}]   ;; load second variable", var_labels.get(var_name).unwrap()).unwrap();
                            
                            match operator {
                                crate::parse::MathOperator::Add => {
                                    writeln!(&mut text_section, "    add rax, rcx   ;; add variables").unwrap();
                                },
                                crate::parse::MathOperator::Subtract => {
                                    writeln!(&mut text_section, "    sub rax, rcx   ;; subtract variables").unwrap();
                                },
                                crate::parse::MathOperator::Multiply => {
                                    writeln!(&mut text_section, "    imul rax, rcx   ;; multiply variables").unwrap();
                                },
                                crate::parse::MathOperator::Divide => {
                                    writeln!(&mut text_section, "    cqo   ;; sign-extend RAX to RDX:RAX for division").unwrap();
                                    writeln!(&mut text_section, "    idiv rcx   ;; divide RDX:RAX by RCX").unwrap();
                                },
                            }
                        } else {
                            writeln!(&mut text_section, "    ;; Warning: variable '{}' not found, operation skipped", var_name).unwrap();
                        }
                    },
                    _ => {
                        writeln!(&mut text_section, "    ;; Complex expression in math operation - not implemented").unwrap();
                    }
                }
                
                // Store the result back in the variable
                writeln!(&mut text_section, "    mov [{}], rax   ;; store result in variable", var_labels.get(name).unwrap()).unwrap();
            },
            ASTNode::Input { name } => {
                // Implement reading input from console
                writeln!(&mut text_section, "    ;; Reading input into variable '{}'", name).unwrap();
                
                if var_labels.contains_key(name) {
                    if let Some(var_type) = var_types.get(name) {
                        match var_type {
                            VarType::String => {
                                // For string variables
                                writeln!(&mut text_section, "    ;; Reading string input").unwrap();
                                writeln!(&mut text_section, "    mov rax, 0   ;; syscall: read").unwrap();
                                writeln!(&mut text_section, "    mov rdi, 0   ;; file descriptor: stdin").unwrap();
                                writeln!(&mut text_section, "    lea rsi, [{}]   ;; buffer to store input", var_labels.get(name).unwrap()).unwrap();
                                writeln!(&mut text_section, "    mov rdx, 255   ;; max bytes to read").unwrap();
                                writeln!(&mut text_section, "    syscall   ;; call read syscall").unwrap();
                                // Trim newline if present
                                writeln!(&mut text_section, "    lea rdi, [{}]   ;; prepare string for trimming", var_labels.get(name).unwrap()).unwrap();
                                writeln!(&mut text_section, "    call trim_newline   ;; remove trailing newline").unwrap();
                            },
                            VarType::Int | VarType::Float => {
                                // For numeric variables
                                writeln!(&mut text_section, "    ;; Reading numeric input").unwrap();
                                // First read as string
                                writeln!(&mut text_section, "    mov rax, 0   ;; syscall: read").unwrap();
                                writeln!(&mut text_section, "    mov rdi, 0   ;; file descriptor: stdin").unwrap();
                                writeln!(&mut text_section, "    mov rsi, num_buffer   ;; buffer to store input").unwrap();
                                writeln!(&mut text_section, "    mov rdx, 19   ;; max bytes to read").unwrap();
                                writeln!(&mut text_section, "    syscall   ;; call read syscall").unwrap();
                                // Trim newline
                                writeln!(&mut text_section, "    mov rdi, num_buffer   ;; prepare string for trimming").unwrap();
                                writeln!(&mut text_section, "    call trim_newline   ;; remove trailing newline").unwrap();
                                // Convert string to integer
                                writeln!(&mut text_section, "    ;; Convert string to integer").unwrap();
                                writeln!(&mut text_section, "    mov rsi, num_buffer   ;; buffer with number string").unwrap();
                                writeln!(&mut text_section, "    xor rax, rax   ;; clear accumulator").unwrap();
                                writeln!(&mut text_section, "    xor rcx, rcx   ;; clear counter").unwrap();
                                writeln!(&mut text_section, ".convert_loop_{}:", name).unwrap();
                                writeln!(&mut text_section, "    mov bl, [rsi + rcx]   ;; get next character").unwrap();
                                writeln!(&mut text_section, "    cmp bl, 0   ;; check for end of string").unwrap();
                                writeln!(&mut text_section, "    je .done_convert_{}   ;; if at end, we're done", name).unwrap();
                                writeln!(&mut text_section, "    sub bl, '0'   ;; convert ASCII to digit value").unwrap();
                                writeln!(&mut text_section, "    imul rax, 10   ;; multiply accumulator by 10").unwrap();
                                writeln!(&mut text_section, "    add rax, rbx   ;; add new digit").unwrap();
                                writeln!(&mut text_section, "    inc rcx   ;; move to next character").unwrap();
                                writeln!(&mut text_section, "    jmp .convert_loop_{}", name).unwrap();
                                writeln!(&mut text_section, ".done_convert_{}:", name).unwrap();
                                // Store result in variable
                                writeln!(&mut text_section, "    mov [{}], rax   ;; store numeric value in variable", var_labels.get(name).unwrap()).unwrap();
                            },
                            _ => {
                                writeln!(&mut text_section, "    ;; Unsupported variable type for input").unwrap();
                            }
                        }
                    }
                } else {
                    writeln!(&mut text_section, "    ;; Error: variable '{}' not declared", name).unwrap();
                }
            },
            ASTNode::If { .. } => {
                // Skip If nodes during the variable declaration phase
                // This will be processed in a separate pass
            },
            _ => {
                writeln!(&mut text_section, "    ;; Unhandled AST node type").unwrap();
            }
        }
    }
    
    // Add exit syscall at the end
    writeln!(&mut text_section, "\n    ;; Exiting program").unwrap();
    writeln!(&mut text_section, "    mov rax, 60   ;; syscall: exit").unwrap();
    writeln!(&mut text_section, "    xor rdi, rdi   ;; exit status 0").unwrap();
    writeln!(&mut text_section, "    syscall").unwrap();
    
    // Add helper functions
    text_section.push_str("\nprint_int:\n");
    text_section.push_str("    mov rsi, num_buffer + 19   ;; set pointer to end of num_buffer\n");
    text_section.push_str("    mov byte [rsi], 0   ;; null-terminate\n");
    text_section.push_str("    mov rbx, 10   ;; divisor for conversion\n");
    text_section.push_str(".int_to_str:\n");
    text_section.push_str("    dec rsi   ;; move pointer left\n");
    text_section.push_str("    xor rdx, rdx   ;; clear remainder register\n");
    text_section.push_str("    div rbx   ;; divide rax by 10\n");
    text_section.push_str("    add dl, '0'   ;; convert remainder to ASCII digit\n");
    text_section.push_str("    mov [rsi], dl   ;; store digit\n");
    text_section.push_str("    test rax, rax   ;; check if quotient is zero\n");
    text_section.push_str("    jnz .int_to_str   ;; loop if not zero\n");
    text_section.push_str("    mov rax, 1   ;; syscall: write\n");
    text_section.push_str("    mov rdi, 1   ;; file descriptor: stdout\n");
    text_section.push_str("    mov rdx, num_buffer + 19\n");
    text_section.push_str("    sub rdx, rsi   ;; compute length of converted string\n");
    text_section.push_str("    syscall   ;; write string\n");
    text_section.push_str("    ret\n\n");
    
    text_section.push_str("print_str:\n");
    text_section.push_str("    mov rcx, 0   ;; initialize counter\n");
    text_section.push_str(".count_loop:\n");
    text_section.push_str("    cmp byte [rsi + rcx], 0   ;; check for null terminator\n");
    text_section.push_str("    je .check_newline\n");
    text_section.push_str("    inc rcx\n");
    text_section.push_str("    jmp .count_loop\n");
    text_section.push_str(".check_newline:\n");
    text_section.push_str("    cmp rcx, 0\n");
    text_section.push_str("    je .done\n");
    text_section.push_str("    dec rcx\n");
    text_section.push_str("    cmp byte [rsi + rcx], 0x0A   ;; check if last character is newline\n");
    text_section.push_str("    jne .print\n");
    text_section.push_str("    jmp .done\n");
    text_section.push_str(".print:\n");
    text_section.push_str("    inc rcx\n");
    text_section.push_str("    mov rax, 1\n");
    text_section.push_str("    mov rdi, 1\n");
    text_section.push_str("    mov rdx, rcx   ;; length of string\n");
    text_section.push_str("    syscall\n");
    text_section.push_str(".done:\n");
    text_section.push_str("    ret\n\n");
    
    text_section.push_str("trim_newline:\n");
    text_section.push_str("    push rbx\n");
    text_section.push_str("    mov rcx, 0\n");
    text_section.push_str(".trim_loop:\n");
    text_section.push_str("    mov al, [rdi + rcx]\n");
    text_section.push_str("    cmp al, 0   ;; end of string check\n");
    text_section.push_str("    je .done\n");
    text_section.push_str("    cmp al, 0x0A   ;; check for newline\n");
    text_section.push_str("    je .replace\n");
    text_section.push_str("    inc rcx\n");
    text_section.push_str("    jmp .trim_loop\n");
    text_section.push_str(".replace:\n");
    text_section.push_str("    mov byte [rdi + rcx], 0   ;; replace newline with null\n");
    text_section.push_str(".done:\n");
    text_section.push_str("    pop rbx\n");
    text_section.push_str("    ret\n");
    
    // Combine all sections
    format!("{}\n{}\n{}", data_section, bss_section, text_section)
}
