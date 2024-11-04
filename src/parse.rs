use crate::types::Types;
use std::collections::HashSet;

#[allow(unused)]
pub fn pven(code: String) -> Vec<Types> {
    let mut ts: Vec<Types> = Vec::new();
    let mut defined_vars: HashSet<String> = HashSet::new();
    let mut used_vars: HashSet<String> = HashSet::new();

    for ln in code.lines() {
        let ln = ln.trim();
        let ln = if let Some(pos) = ln.find(';') { &ln[..pos].trim() } else { ln };

        if ln.is_empty() {
            continue;
        }

        if ln.starts_with("> ") {
            let text = &ln[2..].trim();
            let mut open_quotes = false;
            let mut segment = String::new();
            let mut collecting_var = false;

            for ch in text.chars() {
                match ch {
                    '\'' => {
                        open_quotes = !open_quotes;
                        segment.push(ch);
                        if !open_quotes && !segment.is_empty() {
                            ts.push(Types::Print(segment.clone()));
                            segment.clear();
                        }
                    },
                    ',' if !open_quotes => {
                        if collecting_var {
                            let trimmed_var = segment.trim().to_string();
                            if trimmed_var == "\n" || trimmed_var == "0x0A" {
                                ts.push(Types::Print(trimmed_var.clone()));
                            } else if defined_vars.contains(&trimmed_var) {
                                used_vars.insert(trimmed_var.clone());
                                ts.push(Types::PVarUse(trimmed_var.clone()));
                            }
                            segment.clear();
                            collecting_var = false;
                        }
                    },
                    _ => {
                        if open_quotes {
                            segment.push(ch);
                        } else if ch.is_alphanumeric() || ch == '_' {
                            segment.push(ch);
                            collecting_var = true;
                        } else if collecting_var {
                            let trimmed_var = segment.trim().to_string();
                            if defined_vars.contains(&trimmed_var) {
                                used_vars.insert(trimmed_var.clone());
                                ts.push(Types::PVarUse(trimmed_var.clone()));
                            }
                            segment.clear();
                            collecting_var = false;
                        }
                    }
                }
            }

            if open_quotes && !segment.is_empty() {
                ts.push(Types::Print(segment.clone()));
            } else if collecting_var && !segment.is_empty() {
                let trimmed_var = segment.trim().to_string();
                if trimmed_var == "\n" || trimmed_var == "0x0A" {
                    ts.push(Types::Print(trimmed_var.clone()));
                } else if defined_vars.contains(&trimmed_var) {
                    used_vars.insert(trimmed_var.clone());
                    ts.push(Types::PVarUse(trimmed_var.clone()));
                }
            }
        } else if ln.starts_with("@ ") || ln.starts_with("!@ ") {
            let is_multivar = ln.starts_with("!@ ");
            let clean_line = if is_multivar { ln.trim_start_matches("!@") } else { ln.trim_start_matches('@') }.trim();
            let mut parts = clean_line.split_whitespace();

            if let (Some(var_name), Some(var_type)) = (parts.next(), parts.next()) {
                let value = parts.collect::<Vec<&str>>().join(" ");
                let var_type_string = match var_type {
                    "i" => "num".to_string(),
                    "f" => "dec".to_string(),
                    "str" => "txt".to_string(),
                    _ => continue,
                };
                defined_vars.insert(var_name.to_string());
                if is_multivar {
                    ts.push(Types::MVar(var_name.to_string(), value, var_type_string));
                } else {
                    ts.push(Types::SVar(var_name.to_string(), value, var_type_string));
                }
            }
        }
    }

    ts.retain(|var| {
        match var {
            Types::SVar(name, _, _) | Types::MVar(name, _, _) => defined_vars.contains(name) && used_vars.contains(name),
            _ => true,
        }
    });

    ts
}
