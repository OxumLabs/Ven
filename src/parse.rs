use crate::errs::VarError;
use crate::impl_parserstate::ParserState;
use crate::token::{Token, TokenKind};

use std::collections::HashMap;

/// ------------------ AST Definitions ------------------

#[derive(Debug, Clone)]
pub enum VarType {
    Int,
    String,
    Char { size: usize },
    Float,
}

#[derive(Debug, Clone)]
pub enum Expression {
    Literal(String),
    Identifier(String),
    BinaryOp {
        left: Box<Expression>,
        operator: ComparisonOperator,
        right: Box<Expression>,
    },
    LogicalOp {
        left: Box<Expression>,
        operator: LogicalOperator,
        right: Box<Expression>,
    },
}

/// Represents comparison operators for conditionals.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComparisonOperator {
    Equal,            // ==
    NotEqual,         // !=
    LessThan,         // <
    LessThanEqual,    // <=
    GreaterThan,      // >
    GreaterThanEqual, // >=
}

/// Represents logical operators for conditionals.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LogicalOperator {
    And, // &&
    Or,  // ||
}

/// Represents the available math operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MathOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

/// Extended AST node variants.
#[derive(Debug, Clone)]
pub enum ASTNode {
    /// Variable declaration.
    /// E.g.: "@ myVar i 42" (static) or "@@ myMutVar str hello" (mutable)
    VarDeclaration {
        mutable: bool,
        name: String,
        var_type: VarType,
        value: Option<Expression>,
    },
    /// Input statement.
    /// E.g.: "> myVar"
    Input { name: String },
    /// Print statement.
    /// E.g.: ">> Hello {name}!"
    Print {
        to_stderr: bool,
        expr: Option<Expression>,
    },
    /// Math operation on a mutable variable.
    /// E.g.: "* age + 2" or "* age / 3"
    MathOp {
        name: String,
        operator: MathOperator,
        operand: Expression,
    },
    /// Represents a conditional statement.
    /// E.g.: "?(age > 18) { >> Adult }"
    /// condition: The condition to evaluate.
    /// body: The statements to execute if the condition is true.
    If {
        condition: Expression,
        body: Vec<ASTNode>,
    },
}

/// The top-level AST wrapping a program.
#[derive(Debug)]
pub enum AST {
    Program(Vec<ASTNode>),
}

impl AST {
    #[inline(always)]
    pub fn parse(
        tokens: &[Token],
        input: &str,
    ) -> (Self, HashMap<String, (VarType, usize)>, Vec<VarError>) {
        let mut nodes = Vec::with_capacity(tokens.len() / 4);
        let mut global_var_map = HashMap::with_capacity(16);
        let mut global_errors = Vec::with_capacity(8);

        // Use a single parser state to iterate over all tokens
        let mut state = ParserState {
            tokens,
            input,
            pos: 0,
            global_var_map: &mut global_var_map,
            errors: Vec::with_capacity(4),
            global_pos: 0,
        };

        while state.pos < tokens.len() {
            let token_kind = state.tokens.get(state.pos).map(|t| t.kind);
            println!("DEBUG PARSE: Trying to parse token at position {} of type {:?}", 
                     state.pos, token_kind);
            
            // Skip newline tokens
            if let Some(TokenKind::Newline) = token_kind {
                println!("DEBUG PARSE: Skipping Newline at position {}", state.pos);
                state.pos += 1;
                continue;
            }
            
            if let Some(stmt) = state.parse_statement(tokens) {
                println!("DEBUG PARSE: Successfully parsed {:?}", stmt);
                nodes.push(stmt);
            } else {
                println!("DEBUG PARSE: Failed to parse token at position {}, skipping", state.pos);
                state.pos += 1;
            }
        }
        
        global_errors.extend(state.errors);
        (AST::Program(nodes), global_var_map, global_errors)
    }

    #[inline(always)]
    pub fn debug(&self) {
        // Explicitly matching self as a reference.
        match self {
            AST::Program(nodes) => {
                println!("AST Tree:");
                for (i, node) in nodes.iter().enumerate() {
                    let is_last = i == nodes.len() - 1;
                    Self::print_node(node, "", is_last);
                }
                println!("End of AST Tree");
            }
        }
    }

    #[inline(always)]
    fn print_node(node: &ASTNode, indent: &str, is_last: bool) {
        let branch = if is_last { "└── " } else { "├── " };
        match node {
            ASTNode::If { condition, body } => {
                println!("{}{}Condition", indent, branch);
                let child_indent = if is_last {
                    format!("{}    ", indent)
                } else {
                    format!("{}│   ", indent)
                };
                println!("{}├── Condition:", child_indent);
                let cond_indent = format!("{}    ", child_indent);
                Self::print_expression(condition, &cond_indent, false);
                println!("{}└── Body:", child_indent);
                let body_indent = format!("{}    ", child_indent);
                for (i, node) in body.iter().enumerate() {
                    let is_last_body = i == body.len() - 1;
                    Self::print_node(node, &body_indent, is_last_body);
                }
            }
            ASTNode::VarDeclaration {
                mutable,
                name,
                var_type,
                value,
            } => {
                println!("{}{}VarDeclaration", indent, branch);
                let child_indent = if is_last {
                    format!("{}    ", indent)
                } else {
                    format!("{}│   ", indent)
                };
                println!("{}├── Name: {}", child_indent, name);
                println!("{}├── Mutable: {}", child_indent, mutable);
                println!("{}└── Type: {:?}", child_indent, var_type);
                if let Some(expr) = value {
                    println!("{}    └── Value:", child_indent);
                    let expr_indent = format!("{}        ", child_indent);
                    Self::print_expression(expr, &expr_indent, true);
                }
            }
            ASTNode::Input { name } => {
                println!("{}{}Input", indent, branch);
                let child_indent = if is_last {
                    format!("{}    ", indent)
                } else {
                    format!("{}│   ", indent)
                };
                println!("{}    └── Name: {}", child_indent, name);
            }
            ASTNode::Print { to_stderr, expr } => {
                println!("{}{}Print", indent, branch);
                let child_indent = if is_last {
                    format!("{}    ", indent)
                } else {
                    format!("{}│   ", indent)
                };
                println!("{}├── to_stderr: {}", child_indent, to_stderr);
                if let Some(expr) = expr {
                    println!("{}└── Expression:", child_indent);
                    let expr_indent = format!("{}    ", child_indent);
                    Self::print_expression(expr, &expr_indent, true);
                }
            }
            ASTNode::MathOp {
                name,
                operator,
                operand,
            } => {
                println!("{}{}MathOp", indent, branch);
                let child_indent = if is_last {
                    format!("{}    ", indent)
                } else {
                    format!("{}│   ", indent)
                };
                println!("{}├── Name: {}", child_indent, name);
                println!("{}├── Operator: {:?}", child_indent, operator);
                println!("{}└── Operand:", child_indent);
                let expr_indent = format!("{}    ", child_indent);
                Self::print_expression(operand, &expr_indent, true);
            }
        }
    }

    #[inline(always)]
    fn print_expression(expr: &Expression, indent: &str, is_last: bool) {
        let branch = if is_last { "└── " } else { "├── " };
        match expr {
            Expression::Literal(lit) => println!("{}{}Literal: {}", indent, branch, lit),
            Expression::Identifier(id) => println!("{}{}Identifier: {}", indent, branch, id),
            Expression::BinaryOp { left, operator, right } => {
                println!("{}{}BinaryOp", indent, branch);
                let child_indent = if is_last {
                    format!("{}    ", indent)
                } else {
                    format!("{}│   ", indent)
                };
                println!("{}├── Left:", child_indent);
                Self::print_expression(left, &child_indent, false);
                println!("{}├── Operator: {:?}", child_indent, operator);
                println!("{}└── Right:", child_indent);
                Self::print_expression(right, &child_indent, true);
            }
            Expression::LogicalOp { left, operator, right } => {
                println!("{}{}LogicalOp", indent, branch);
                let child_indent = if is_last {
                    format!("{}    ", indent)
                } else {
                    format!("{}│   ", indent)
                };
                println!("{}├── Left:", child_indent);
                Self::print_expression(left, &child_indent, false);
                println!("{}├── Operator: {:?}", child_indent, operator);
                println!("{}└── Right:", child_indent);
                Self::print_expression(right, &child_indent, true);
            }
        }
    }
}

pub type VarMap = HashMap<String, (VarType, usize)>;
