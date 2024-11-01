use crate::types::Types;

pub fn lfor64(tokens: Vec<Types>) -> String {
    let mut asm_code = String::new();
    asm_code.push_str("section .data\n");

    let mut msglb = 0;
    let mut messages: Vec<String> = Vec::new();
    let mut message_lengths: Vec<String> = Vec::new();

    for token in tokens {
        match token {
            Types::Print(text) => {
                let message_label = format!("message_{}", msglb);
                let message_len_label = format!("message_len_{}", msglb);

                messages.push(format!("{} db {}, 0", message_label, text));
                message_lengths.push(format!("{} equ $ - {}", message_len_label, message_label));

                msglb += 1;
            }
            Types::SVar(name, value, vtype) => match vtype.as_str() {
                "txt" => {
                    asm_code.push_str(&format!("{} db '{}', 0\n", name, value));
                }
                "num" => {
                    asm_code.push_str(&format!("{} dd {}\n", name, value));
                }
                "dec" => {
                    asm_code.push_str(&format!("{} dq {}\n", name, value));
                }
                _ => {}
            },
            Types::MVar(name, value, vtype) => match vtype.as_str() {
                "txt" => {
                    asm_code.push_str(&format!("{} db '{}', 0\n", name, value));
                }
                "num" => {
                    asm_code.push_str(&format!("{} dd {}\n", name, value));
                }
                "dec" => {
                    asm_code.push_str(&format!("{} dq {}\n", name, value));
                }
                _ => {}
            },
        }
    }

    for message in messages {
        asm_code.push_str(&message);
        asm_code.push_str("\n");
    }

    for length in message_lengths {
        asm_code.push_str(&length);
        asm_code.push_str("\n");
    }

    asm_code.push_str("\nsection .text\n");
    asm_code.push_str("global _start\n\n");
    asm_code.push_str("_start:\n");

    for i in 0..msglb {
        let message_label = format!("message_{}", i);
        let message_len_label = format!("message_len_{}", i);

        asm_code.push_str("    mov rax, 1\n");
        asm_code.push_str("    mov rdi, 1\n");
        asm_code.push_str(&format!("    mov rsi, {}\n", message_label));
        asm_code.push_str(&format!("    mov rdx, {}\n", message_len_label));
        asm_code.push_str("    syscall\n\n");
    }

    asm_code.push_str("    mov rax, 60\n");
    asm_code.push_str("    xor rdi, rdi\n");
    asm_code.push_str("    syscall\n");

    asm_code
}
