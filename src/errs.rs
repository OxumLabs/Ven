#[derive(Debug, Clone)]
pub enum VarError {
    /// Indicates that a variable was used but not declared.
    UndeclaredVariable { name: String, line: usize },
    
    /// Indicates that the literal value's type does not match the variable's declared type.
    TypeMismatch {
        expected: String,
        found: String,
        line: usize,
    },
    
    /// Indicates that assigning to an immutable variable.
    ImmutableAssignment { name: String, line: usize },
    
    /// Expected '(' after '?' but found none.
    MissingConditionOpenParen { line: usize },
    
    /// Expected ')' but found none.
    MissingConditionCloseParen { line: usize },
    
    /// Expected '{' but found none.
    MissingBlockOpenBrace { line: usize },
    
    /// Unmatched closing brace.
    UnmatchedClosingBrace { line: usize },
    
    /// Invalid condition
    InvalidCondition { details: String, line: usize },
}
