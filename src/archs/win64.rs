use crate::types::Types;

pub fn w64(tokens: Vec<Types>) -> String {
    let mut asm_code = String::new();
    asm_code.push_str("section .data\n");

    let mut message_declarations = Vec::new();
    let mut length_declarations = Vec::new();

    asm_code.push_str("section .text\n");
    asm_code.push_str("global mainCRTStartup\n\n");
    asm_code.push_str("mainCRTStartup:\n"); // Correct entry point

    for (i, token) in tokens.iter().enumerate() {
        match token {
            Types::Print(text) => {
                let message_label = format!("message_{}", i);
                message_declarations.push(format!("{} db {}, 0\n", message_label, text));
                length_declarations.push(format!("length_{} equ $ - {}\n", i, message_label));

                // Use Windows API for output
                asm_code.push_str(&format!("    mov rdx, length_{}\n    mov rsi, {}\n", i, message_label));
                asm_code.push_str("    mov rax, 1\n"); // File descriptor for stdout
                asm_code.push_str("    mov rdi, 1\n"); // File descriptor for stdout
                asm_code.push_str("    syscall\n\n");
            }
            Types::SVar(name, value, vtype) | Types::MVar(name, value, vtype) => {
                match vtype.as_str() {
                    "txt" => {
                        asm_code.push_str(&format!("{} db '{}', 0\n", name, value));
                    }
                    "num" | "dec" => {
                        asm_code.push_str(&format!("{} dq {}\n", name, value));
                    }
                    _ => {}
                }
            }
        }
    }

    for declaration in message_declarations {
        asm_code.push_str(&declaration);
    }
    for length in length_declarations {
        asm_code.push_str(&length);
    }

    asm_code.push_str("    xor rdi, rdi\n"); // Set exit code to 0
    asm_code.push_str("    mov rax, 60\n"); // syscall for exit
    asm_code.push_str("    syscall\n"); // invoke syscall

    asm_code
}
