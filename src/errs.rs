#[derive(Debug,Clone)]
pub enum VarError {
    /// Indicates that the literal value’s type does not match the variable's declared type.
    TypeMismatch {
        var_name: String,
        expected: String,
        found: String,
        line: usize,
    },
    /// Indicates that the literal value could not be parsed into the expected type.
    VarTypeMisMatch {
        var_name: String,
        expected: String,
        value: String,
        line: usize,
    },
    /// Indicates that a variable was used but not declared.
    Undeclared {
        var_name: String,
        line: usize,
    },
}
