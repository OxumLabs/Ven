use crate::types::Types;

#[allow(unreachable_patterns)]
pub fn lfor32(tokens: Vec<Types>) -> String {
    let mut asm_code = String::new();
    asm_code.push_str("section .data\n\n");

    let mut msglb = 0;
    let mut messages: Vec<(String, String)> = Vec::new();

    // Parse the tokens and prepare messages and variable declarations
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

    // Add message definitions to the assembly code
    for (_, msg) in &messages {
        if msg.trim().starts_with("message") {
            asm_code.push_str(&format!("{}\n", msg));
        }
    }

    asm_code.push_str("\nsection .text\n");
    asm_code.push_str("global _start\n\n");
    asm_code.push_str("_start:\n");

    // Generate the write calls for each message
    for (name, _) in messages.iter() {
        asm_code.push_str("    mov eax, 4          ; sys_write\n");
        asm_code.push_str("    mov ebx, 1          ; stdout\n");
        asm_code.push_str(&format!("    mov ecx, {}\n", name));
        
        // Calculate the length of the string
        asm_code.push_str("    mov edx, 0          ; Initialize length\n");
        asm_code.push_str(&format!("    find_length_{}:\n", msglb));
        asm_code.push_str("        cmp byte [ecx + edx], 0\n");
        asm_code.push_str(&format!("        je done_length_{}\n", msglb));
        asm_code.push_str("        inc edx\n");
        asm_code.push_str(&format!("        jmp find_length_{}\n", msglb));
        asm_code.push_str(&format!("    done_length_{}:\n", msglb));
        
        // Write the message
        asm_code.push_str("    int 0x80            ; Call kernel\n\n");
        msglb += 1;
    }

    // Exit the program
    asm_code.push_str("    mov eax, 1          ; sys_exit\n");
    asm_code.push_str("    xor ebx, ebx\n");
    asm_code.push_str("    int 0x80\n");

    asm_code
}
