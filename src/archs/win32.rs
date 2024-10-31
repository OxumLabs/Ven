use crate::types::Types;

pub fn w32(tokens: Vec<Types>) -> String {
    let mut asm_code = String::new();

    let mut message_declarations = Vec::new();
    let mut length_declarations = Vec::new();

    // Start building the .data section
    asm_code.push_str("section .data\n");
    asm_code.push_str("stdout dd 1\n"); // Standard output handle

    // Start building the .text section
    asm_code.push_str("section .text\n");
    asm_code.push_str("global _mainCRTStartup\n\n");
    asm_code.push_str("_mainCRTStartup:\n"); // Correct entry point

    for (i, token) in tokens.iter().enumerate() {
        match token {
            Types::Print(text) => {
                let message_label = format!("message_{}", i);
                // Append message and length declarations
                message_declarations.push(format!("{} db {}, 0\n", message_label, text));
                length_declarations.push(format!("length_{} equ $ - {}\n", i, message_label));

                // Prepare the call to WriteFile
                asm_code.push_str(&format!("    mov edx, length_{} ; Length of message\n", i));
                asm_code.push_str(&format!("    mov ecx, {}\n", message_label));
                asm_code.push_str("    mov ebx, [stdout]\n"); // Load stdout handle
                asm_code.push_str("    mov eax, 4\n"); // syscall number for sys_write
                asm_code.push_str("    int 0x80\n\n"); // Call the syscall
            }
            Types::SVar(name, value, vtype) | Types::MVar(name, value, vtype) => {
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
                    _ => {}
                }
            }
        }
    }

    // Append the collected message declarations
    for declaration in message_declarations {
        asm_code.push_str(&declaration);
    }
    
    // Append the collected length declarations
    for length in length_declarations {
        asm_code.push_str(&length);
    }

    // Exit process
    asm_code.push_str("    xor eax, eax\n"); // Set exit code to 0
    asm_code.push_str("    mov eax, 1\n"); // syscall number for sys_exit
    asm_code.push_str("    int 0x80\n"); // invoke syscall

    asm_code
}
