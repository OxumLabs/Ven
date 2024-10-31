use crate::types::Types;

#[allow(unused)]
pub fn pven(code: String) -> Vec<Types> {
    let mut ts: Vec<Types> = Vec::new();

    for ln in code.lines() {
        let ln = ln.trim();
        let ln = if let Some(pos) = ln.find(';') {
            &ln[..pos].trim()
        } else {
            ln
        };

        if ln.is_empty() {
            continue;
        }

        if ln.starts_with("> ") {
            let text = &ln[2..].trim();
            let mut open_quotes = 0;
            let mut is_valid = true;
            let mut processed_text = String::new();

            for (i, ch) in text.chars().enumerate() {
                if ch == '\'' {
                    open_quotes += 1;
                    if i > 0 && text.chars().nth(i - 1) == Some('\\') {
                        continue;
                    }
                }
                processed_text.push(ch);
            }

            if open_quotes % 2 != 0 {
                is_valid = false;
            }

            if is_valid {
                ts.push(Types::Print(processed_text));
            } else {
                println!("Failed to process: {}", text);
            }
        } else if ln.starts_with("@ ") {
            let clean_line = ln.trim_start_matches('@').trim();
            let mut parts = clean_line.split_whitespace();

            if let (Some(var_name), Some(var_type)) = (parts.next(), parts.next()) {
                let value = parts.collect::<Vec<&str>>().join(" ");
                let var_type_string = match var_type {
                    "i" => "num".to_string(),
                    "f" => "dec".to_string(),
                    "str" => "txt".to_string(),
                    _ => continue,
                };
                ts.push(Types::SVar(var_name.to_string(), value, var_type_string));
            }
        } else if ln.starts_with("!@ ") {
            let clean_line = ln.trim_start_matches("!@").trim();
            let mut parts: Vec<&str> = clean_line.split_whitespace().collect();

            if parts.len() < 3 {
                continue;
            }

            let var_name = parts[0];
            let var_type = parts[1];
            let value = parts[2..].join(" ");
            let var_type_string = match var_type {
                "i" => "num".to_string(),
                "f" => "dec".to_string(),
                "str" => "txt".to_string(),
                _ => continue,
            };
            ts.push(Types::MVar(var_name.to_string(), value, var_type_string));
        }
    }

    ts
}
