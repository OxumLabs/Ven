use crate::types::Types;

pub fn m64(tokens: Vec<Types>) -> String {
    let mut asm_code = String::new();
    asm_code.push_str("section .data\n");
    asm_code.push_str("section .text\n");
    asm_code.push_str("global _start\n\n");
    asm_code.push_str("_start:\n");

    for token in tokens {
        match token {
            Types::Print(text) => {
                asm_code.push_str("    mov rax, 0x2000004\n");
                asm_code.push_str("    mov rdi, 1\n");
                asm_code.push_str("    mov rsi, message\n");
                asm_code.push_str("    mov rdx, message_len\n");
                asm_code.push_str("    syscall\n\n");

                asm_code.push_str(&format!("message db '{}', 0\n", text));
                asm_code.push_str("message_len equ $ - message\n");
            }
            Types::SVar(name, value, vtype) => {
                match vtype.as_str() {
                    "txt" => {
                        asm_code.push_str(&format!("{} db '{}', 0\n", name, value));
                    }
                    "num" => {
                        asm_code.push_str(&format!("{} dd {}\n", name, value));
                    }
                    "dec" => {
                        asm_code.push_str(&format!("{} dq {}\n", name, value));
                    }
                    _ => {
                        eprintln!("Unsupported variable type: {}", vtype);
                    }
                }
            },
            Types::MVar(name, value, vtype) => {
                match vtype.as_str() {
                    "txt" => {
                        asm_code.push_str(&format!("{} db '{}', 0\n", name, value));
                    }
                    "num" => {
                        asm_code.push_str(&format!("{} dd {}\n", name, value));
                    }
                    "dec" => {
                        asm_code.push_str(&format!("{} dq {}\n", name, value));
                    }
                    _ => {
                        eprintln!("Unsupported mutable variable type: {}", vtype);
                    }
                }
            }
        }
    }

    asm_code.push_str("    mov rax, 0x2000001\n");
    asm_code.push_str("    xor rdi, rdi\n");
    asm_code.push_str("    syscall\n");

    asm_code
}
