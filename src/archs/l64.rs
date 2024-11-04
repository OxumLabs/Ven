use crate::types::Types;

#[allow(unreachable_patterns)]
pub fn lfor64(tokens: Vec<Types>) -> String {
    let mut asm_code = String::new();
    asm_code.push_str("section .data\n\n");

    let mut msglb = 0;
    let mut messages: Vec<(String, String)> = Vec::new();

    for token in tokens.iter() {
        match token {
            Types::Print(text) => {
                let message_label = format!("message_{}", msglb);
                let message_str = format!("    {} db {}, 0", message_label, text);
                messages.push((message_label.clone(), message_str.clone()));
                msglb += 1;
            }
            Types::SVar(name, value, vtype) | Types::MVar(name, value, vtype) => {
                match vtype.as_str() {
                    "txt" => {
                        let var_declaration = format!("    {} db '{}', 0", name, value);
                        asm_code.push_str(&var_declaration);
                        asm_code.push_str("\n");
                    }
                    "num" => {
                        let var_declaration = format!("    {} dd {}", name, value);
                        asm_code.push_str(&var_declaration);
                        asm_code.push_str("\n");
                    }
                    "dec" => {
                        let var_declaration = format!("    {} dq {}", name, value);
                        asm_code.push_str(&var_declaration);
                        asm_code.push_str("\n");
                    }
                    _ => {}
                }
            }
            Types::PVarUse(name) => {
                messages.push((name.clone(), name.to_string()));
            }
            _ => {}
        }
    }

    for (_, msg) in &messages {
        if msg.trim().starts_with("message") {
            asm_code.push_str(&format!("{}\n", msg));
        }
    }

    asm_code.push_str("\nsection .text\n");
    asm_code.push_str("global _start\n\n");
    asm_code.push_str("_start:\n");

    for (name, _) in messages.iter() {
        asm_code.push_str("    mov rax, 1          ; sys_write\n");
        asm_code.push_str("    mov rdi, 1          ; stdout\n");
        asm_code.push_str(&format!("    mov rsi, {}\n", name));
        asm_code.push_str("    xor rdx, rdx\n");
        asm_code.push_str("    mov rcx, rsi\n");
        
        let length_label = format!("find_length_{}", msglb);
        let done_label = format!("done_length_{}", msglb);
        asm_code.push_str(&format!("    {}:\n", length_label));
        asm_code.push_str("        cmp byte [rcx], 0\n");
        asm_code.push_str(&format!("        je {}\n", done_label));
        asm_code.push_str("        inc rcx\n");
        asm_code.push_str("        inc rdx\n");
        asm_code.push_str(&format!("        jmp {}\n", length_label));
        asm_code.push_str(&format!("{}:\n", done_label));

        asm_code.push_str("    syscall\n\n");
        msglb += 1;
    }

    asm_code.push_str("    mov rax, 60         ; sys_exit\n");
    asm_code.push_str("    xor rdi, rdi\n");
    asm_code.push_str("    syscall\n");

    asm_code
}
