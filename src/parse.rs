use crate::types::Types;

pub fn pven(code: String) -> Vec<Types> {
    let mut ts: Vec<Types> = Vec::new();
    
    for ln in code.lines() {
        let ln = ln.trim();
        
        if ln.starts_with("display ") {
            let text = &ln[8..];
            println!("To Print: {}", text);
            ts.push(Types::Print(text.to_string()));
        } else if ln.starts_with("vr ") {
            let mut parts = ln.split_whitespace();

            if let (Some(var_name), Some(var_type)) = (parts.next(), parts.next()) {
                let value = parts.collect::<Vec<&str>>().join(" ");

                let var_type_string = match var_type {
                    "i" => "num".to_string(),
                    "f" => "dec".to_string(),
                    "str" => "txt".to_string(),
                    _ => {
                        eprintln!("Error: Unsupported variable type: {}", var_type);
                        continue;
                    }
                };

                ts.push(Types::SVar(var_name.to_string(), value, var_type_string));
            } else {
                eprintln!("Error: Invalid variable declaration: {}", ln);
            }
        }
        else if ln.starts_with("mvr ") {
            let mut parts = ln.split_whitespace();

            if let (Some(var_name), Some(var_type)) = (parts.next(), parts.next()) {
                let value = parts.collect::<Vec<&str>>().join(" ");

                let var_type_string = match var_type {
                    "i" => "num".to_string(),
                    "f" => "dec".to_string(),
                    "str" => "txt".to_string(),
                    _ => {
                        eprintln!("Error: Unsupported variable type: {}", var_type);
                        continue;
                    }
                };

                ts.push(Types::MVar(var_name.to_string(), value, var_type_string));
            } else {
                eprintln!("Error: Invalid variable declaration: {}", ln);
            }
        }
    }
    
    ts
}
