use crate::types::Types;

#[allow(unused)]

pub fn pven(code: String) -> Vec<Types> {
    let mut ts: Vec<Types> = Vec::new();

    for ln in code.lines() {
        let ln = ln.trim();

        if ln.starts_with("display ") {
            let text = &ln[8..].trim();
            let mut open_quotes = 0;
            let mut has_comma = false;
            let mut processed_text = String::new();
            let mut is_valid = true;

            for (i, ch) in text.chars().enumerate() {
                if ch == '\'' {
                    open_quotes += 1;
                    if i > 0 && text.chars().nth(i - 1) == Some('\\') {
                        continue;
                    }
                } else if ch == ',' {
                    has_comma = true;
                    if open_quotes % 2 != 0 {
                        is_valid = false;
                        break;
                    }
                } else if ch == '\n' {
                    is_valid = false;
                    break;
                }
                processed_text.push(ch);
            }

            if open_quotes % 2 != 0 {
                is_valid = false;
            }

            if is_valid {
                if processed_text.ends_with(',') {
                    processed_text.pop();
                }
                println!("To Print: {}", processed_text);
                ts.push(Types::Print(processed_text));
            } else {
                println!("Failed to process: {}", text);
            }
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
        } else if ln.starts_with("mvr ") {
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
