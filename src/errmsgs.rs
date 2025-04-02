use crate::errs::VarError;

const _BRANCH_LAST: &str = "└── ";
const _BRANCH_MID: &str = "├── ";
const _INDENT: &str = "    ";
const _EXPL_TYPE_MISMATCH: &str = "The initializer's type does not match the declared type.";
const _EXPL_VAR_TYPE_MISMATCH: &str =
    "The literal value could not be parsed into the expected type.";
const _EXPL_UNDECLARED: &str = "The variable was used before being declared.";
const _EXPL_MISSING_OPEN_PAREN: &str = "Expected '(' after '?' but found none.";
const _EXPL_MISSING_CLOSE_PAREN: &str = "Expected ')' to close the condition but found none.";
const _EXPL_MISSING_OPEN_BRACE: &str = "Expected '{' to start the block but found none.";
const _EXPL_UNMATCHED_CLOSING_BRACE: &str = "Unexpected '}' without a matching opening '{'.";
const _EXPL_UNEXPECTED_TOKEN: &str = "An unexpected token was encountered.";

pub fn print_errors(errors: &[VarError]) {
    if errors.is_empty() {
        return;
    }
    
    for error in errors.iter() {
        match error {
            VarError::UndeclaredVariable { name, line } => {
                println!("\n╭─ Error at line {}{}─╮", line, "─".repeat(50));
                println!("│ Undeclared Variable");
                println!("├{}┤", "─".repeat(65));
                println!("│ Variable '{}' used but not declared", name);
                println!("╰{}╯", "─".repeat(65));
            }
            VarError::TypeMismatch { expected, found, line } => {
                println!("\n╭─ Error at line {}{}─╮", line, "─".repeat(50));
                println!("│ Type Mismatch");
                println!("├{}┤", "─".repeat(65));
                println!("│ Expected type: {}", expected);
                println!("│ Found type: {}", found);
                println!("╰{}╯", "─".repeat(65));
            }
            VarError::ImmutableAssignment { name, line } => {
                println!("\n╭─ Error at line {}{}─╮", line, "─".repeat(50));
                println!("│ Assignment to Immutable Variable");
                println!("├{}┤", "─".repeat(65));
                println!("│ Cannot assign to immutable variable '{}'", name);
                println!("╰{}╯", "─".repeat(65));
            }
            VarError::MissingConditionOpenParen { line } => {
                println!("\n╭─ Error at line {}{}─╮", line, "─".repeat(50));
                println!("│ Missing Opening Parenthesis");
                println!("├{}┤", "─".repeat(65));
                println!("│ Expected '(' after '?' in conditional statement");
                println!("╰{}╯", "─".repeat(65));
            }
            VarError::MissingConditionCloseParen { line } => {
                println!("\n╭─ Error at line {}{}─╮", line, "─".repeat(50));
                println!("│ Missing Closing Parenthesis");
                println!("├{}┤", "─".repeat(65));
                println!("│ Expected ')' to close condition in conditional statement");
                println!("╰{}╯", "─".repeat(65));
            }
            VarError::MissingBlockOpenBrace { line } => {
                println!("\n╭─ Error at line {}{}─╮", line, "─".repeat(50));
                println!("│ Missing Opening Brace");
                println!("├{}┤", "─".repeat(65));
                println!("│ Expected '{{' to start the block but found none");
                println!("╰{}╯", "─".repeat(65));
            }
            VarError::UnmatchedClosingBrace { line } => {
                println!("\n╭─ Error at line {}{}─╮", line, "─".repeat(50));
                println!("│ Unmatched Closing Brace");
                println!("├{}┤", "─".repeat(65));
                println!("│ Found '}}' without matching opening brace");
                println!("╰{}╯", "─".repeat(65));
            }
            VarError::InvalidCondition { details, line } => {
                println!("\n╭─ Error at line {}{}─╮", line, "─".repeat(50));
                println!("│ Invalid Condition");
                println!("├{}┤", "─".repeat(65));
                println!("│ {}", details);
                println!("╰{}╯", "─".repeat(65));
            }
        }
    }
}
