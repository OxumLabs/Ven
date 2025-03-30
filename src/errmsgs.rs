use crate::errs::VarError;

const BRANCH_LAST: &str = "└── ";
const BRANCH_MID: &str = "├── ";
const INDENT: &str = "    ";
const EXPL_TYPE_MISMATCH: &str = "The initializer's type does not match the declared type.";
const EXPL_VAR_TYPE_MISMATCH: &str = "The literal value could not be parsed into the expected type.";
const EXPL_UNDECLARED: &str = "The variable was used before being declared.";

#[inline(always)]
pub fn print_errors(errors: &[VarError]) {
    if errors.is_empty() {
        println!("No errors found.");
        return;
    }
    println!("Errors:");
    for (i, error) in errors.iter().enumerate() {
        let branch = if i == errors.len() - 1 { BRANCH_LAST } else { BRANCH_MID };
        match error {
            VarError::TypeMismatch { var_name, expected, found, line } => {
                let display_line = line + 1;
                println!("{}TypeMismatch", branch);
                println!("{}├── Variable: {}", INDENT, var_name);
                println!("{}├── Expected: {}", INDENT, expected);
                println!("{}├── Found: {}", INDENT, found);
                println!("{}├── Explanation: {}", INDENT, EXPL_TYPE_MISMATCH);
                println!("{}└── Line: {}", INDENT, display_line);
            }
            VarError::VarTypeMisMatch { var_name, expected, value, line } => {
                let display_line = line + 1;
                println!("{}VarTypeMisMatch", branch);
                println!("{}├── Variable: {}", INDENT, var_name);
                println!("{}├── Expected: {}", INDENT, expected);
                println!("{}├── Value: {}", INDENT, value);
                println!("{}├── Explanation: {}", INDENT, EXPL_VAR_TYPE_MISMATCH);
                println!("{}└── Line: {}", INDENT, display_line);
            }
            VarError::Undeclared { var_name, line } => {
                let display_line = line + 1;
                println!("{}Undeclared", branch);
                println!("{}├── Variable: {}", INDENT, var_name);
                println!("{}├── Explanation: {}", INDENT, EXPL_UNDECLARED);
                println!("{}└── Line: {}", INDENT, display_line);
            }
        }
    }
}
