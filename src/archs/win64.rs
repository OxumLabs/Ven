use crate::types::Types;

pub fn w64(tokens: Vec<Types>) -> String {
    let mut asm_code = String::new();
    asm_code.push_str("section .data\n");
    asm_code.push_str("section .text\n");
    asm_code.push_str("extern _ExitProcess@4\n");
    asm_code.push_str("extern _puts\n");
    asm_code.push_str("global _main\n\n");
    asm_code.push_str("_main:\n");

    for token in tokens {
        match token {
            Types::Print(text) => {
                asm_code.push_str("    mov rcx, message\n");
                asm_code.push_str("    call _puts\n\n");

                asm_code.push_str(&format!("message db '{}', 0\n", text));
            }
            Types::SVar(name, value, vtype) => {
                match vtype.as_str() {
                    "txt" => {
                        asm_code.push_str(&format!("{} db '{}', 0\n", name, value));
                    }
                    "num" => {
                        asm_code.push_str(&format!("{} dq {}\n", name, value));
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
                        asm_code.push_str(&format!("{} dq {}\n", name, value));
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

    asm_code.push_str("    call _ExitProcess@4\n");

    asm_code
}
