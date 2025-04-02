DEBUG PARSE: Trying to parse token at position 0 of type Some(Comment)
DEBUG SKIP: Before skip_whitespace pos=0
DEBUG SKIP: After skip_whitespace pos=0
DEBUG STMT: Parsing token with kind Comment and lexeme "; Test file for runtime string interpolation"
DEBUG STMT: Unknown token kind: Comment
DEBUG PARSE: Failed to parse token at position 1, skipping
DEBUG PARSE: Trying to parse token at position 2 of type Some(Newline)
DEBUG PARSE: Skipping Newline at position 2
DEBUG PARSE: Trying to parse token at position 3 of type Some(Comment)
DEBUG SKIP: Before skip_whitespace pos=3
DEBUG SKIP: After skip_whitespace pos=3
DEBUG STMT: Parsing token with kind Comment and lexeme "; Define a mutable variable"
DEBUG STMT: Unknown token kind: Comment
DEBUG PARSE: Failed to parse token at position 4, skipping
DEBUG PARSE: Trying to parse token at position 5 of type Some(At)
DEBUG SKIP: Before skip_whitespace pos=5
DEBUG SKIP: After skip_whitespace pos=5
DEBUG STMT: Parsing token with kind At and lexeme "@"
DEBUG STMT: Found At token (variable declaration)
DEBUG SKIP: Before skip_whitespace pos=7
DEBUG SKIP: After skip_whitespace pos=8
DEBUG SKIP: Before skip_whitespace pos=9
DEBUG SKIP: After skip_whitespace pos=10
DEBUG SKIP: Before skip_whitespace pos=11
DEBUG SKIP: After skip_whitespace pos=12
DEBUG SKIP: Before skip_whitespace pos=12
DEBUG SKIP: After skip_whitespace pos=12
DEBUG PARSE: Successfully parsed VarDeclaration { mutable: true, name: "counter", var_type: Int, value: Some(Literal("0")) }
DEBUG PARSE: Trying to parse token at position 13 of type Some(Newline)
DEBUG PARSE: Skipping Newline at position 13
DEBUG PARSE: Trying to parse token at position 14 of type Some(Newline)
DEBUG PARSE: Skipping Newline at position 14
DEBUG PARSE: Trying to parse token at position 15 of type Some(Comment)
DEBUG SKIP: Before skip_whitespace pos=15
DEBUG SKIP: After skip_whitespace pos=15
DEBUG STMT: Parsing token with kind Comment and lexeme "; Print initial value"
DEBUG STMT: Unknown token kind: Comment
DEBUG PARSE: Failed to parse token at position 16, skipping
DEBUG PARSE: Trying to parse token at position 17 of type Some(Greater)
DEBUG SKIP: Before skip_whitespace pos=17
DEBUG SKIP: After skip_whitespace pos=17
DEBUG STMT: Parsing token with kind Greater and lexeme ">"
DEBUG STMT: Found Greater token (print operation)
DEBUG SKIP: Before skip_whitespace pos=19
DEBUG SKIP: After skip_whitespace pos=20
DEBUG PARSE: Successfully parsed Print { to_stderr: false, expr: Some(Literal("\"Initial counter value: {counter}\"")) }
DEBUG PARSE: Trying to parse token at position 22 of type Some(Newline)
DEBUG PARSE: Skipping Newline at position 22
DEBUG PARSE: Trying to parse token at position 23 of type Some(Comment)
DEBUG SKIP: Before skip_whitespace pos=23
DEBUG SKIP: After skip_whitespace pos=23
DEBUG STMT: Parsing token with kind Comment and lexeme "; Modify the counter"
DEBUG STMT: Unknown token kind: Comment
DEBUG PARSE: Failed to parse token at position 24, skipping
DEBUG PARSE: Trying to parse token at position 25 of type Some(Star)
DEBUG SKIP: Before skip_whitespace pos=25
DEBUG SKIP: After skip_whitespace pos=25
DEBUG STMT: Parsing token with kind Star and lexeme "*"
DEBUG STMT: Found Star token
DEBUG PARSE: Successfully parsed MathOp { name: "counter", operator: Add, operand: Literal("1") }
DEBUG PARSE: Trying to parse token at position 33 of type Some(Newline)
DEBUG PARSE: Skipping Newline at position 33
DEBUG PARSE: Trying to parse token at position 34 of type Some(Comment)
DEBUG SKIP: Before skip_whitespace pos=34
DEBUG SKIP: After skip_whitespace pos=34
DEBUG STMT: Parsing token with kind Comment and lexeme "; Print after modification"
DEBUG STMT: Unknown token kind: Comment
DEBUG PARSE: Failed to parse token at position 35, skipping
DEBUG PARSE: Trying to parse token at position 36 of type Some(Greater)
DEBUG SKIP: Before skip_whitespace pos=36
DEBUG SKIP: After skip_whitespace pos=36
DEBUG STMT: Parsing token with kind Greater and lexeme ">"
DEBUG STMT: Found Greater token (print operation)
DEBUG SKIP: Before skip_whitespace pos=38
DEBUG SKIP: After skip_whitespace pos=39
DEBUG PARSE: Successfully parsed Print { to_stderr: false, expr: Some(Literal("\"Counter after +1: {counter}\"")) }
DEBUG PARSE: Trying to parse token at position 41 of type Some(Newline)
DEBUG PARSE: Skipping Newline at position 41
DEBUG PARSE: Trying to parse token at position 42 of type Some(Comment)
DEBUG SKIP: Before skip_whitespace pos=42
DEBUG SKIP: After skip_whitespace pos=42
DEBUG STMT: Parsing token with kind Comment and lexeme "; Modify again"
DEBUG STMT: Unknown token kind: Comment
DEBUG PARSE: Failed to parse token at position 43, skipping
DEBUG PARSE: Trying to parse token at position 44 of type Some(Star)
DEBUG SKIP: Before skip_whitespace pos=44
DEBUG SKIP: After skip_whitespace pos=44
DEBUG STMT: Parsing token with kind Star and lexeme "*"
DEBUG STMT: Found Star token
DEBUG PARSE: Successfully parsed MathOp { name: "counter", operator: Add, operand: Literal("5") }
DEBUG PARSE: Trying to parse token at position 52 of type Some(Newline)
DEBUG PARSE: Skipping Newline at position 52
DEBUG PARSE: Trying to parse token at position 53 of type Some(Comment)
DEBUG SKIP: Before skip_whitespace pos=53
DEBUG SKIP: After skip_whitespace pos=53
DEBUG STMT: Parsing token with kind Comment and lexeme "; Print after second modification"
DEBUG STMT: Unknown token kind: Comment
DEBUG PARSE: Failed to parse token at position 54, skipping
DEBUG PARSE: Trying to parse token at position 55 of type Some(Greater)
DEBUG SKIP: Before skip_whitespace pos=55
DEBUG SKIP: After skip_whitespace pos=55
DEBUG STMT: Parsing token with kind Greater and lexeme ">"
DEBUG STMT: Found Greater token (print operation)
DEBUG SKIP: Before skip_whitespace pos=57
DEBUG SKIP: After skip_whitespace pos=58
DEBUG PARSE: Successfully parsed Print { to_stderr: false, expr: Some(Literal("\"Counter after +5 more: {counter}\"")) }
DEBUG PARSE: Trying to parse token at position 60 of type Some(Newline)
DEBUG PARSE: Skipping Newline at position 60
DEBUG PARSE: Trying to parse token at position 61 of type Some(Comment)
DEBUG SKIP: Before skip_whitespace pos=61
DEBUG SKIP: After skip_whitespace pos=61
DEBUG STMT: Parsing token with kind Comment and lexeme "; Prompt for user input"
DEBUG STMT: Unknown token kind: Comment
DEBUG PARSE: Failed to parse token at position 62, skipping
DEBUG PARSE: Trying to parse token at position 63 of type Some(Greater)
DEBUG SKIP: Before skip_whitespace pos=63
DEBUG SKIP: After skip_whitespace pos=63
DEBUG STMT: Parsing token with kind Greater and lexeme ">"
DEBUG STMT: Found Greater token (print operation)
DEBUG SKIP: Before skip_whitespace pos=65
DEBUG SKIP: After skip_whitespace pos=66
DEBUG PARSE: Successfully parsed Print { to_stderr: false, expr: Some(Literal("\"Enter a number to add to counter:\"")) }
DEBUG PARSE: Trying to parse token at position 68 of type Some(Greater)
DEBUG SKIP: Before skip_whitespace pos=68
DEBUG SKIP: After skip_whitespace pos=68
DEBUG STMT: Parsing token with kind Greater and lexeme ">"
DEBUG STMT: Found Greater token (print operation)
DEBUG SKIP: Before skip_whitespace pos=69
DEBUG SKIP: After skip_whitespace pos=70
DEBUG SKIP: Before skip_whitespace pos=70
DEBUG SKIP: After skip_whitespace pos=70
DEBUG PARSE: Successfully parsed Print { to_stderr: false, expr: Some(Identifier("counter")) }
DEBUG PARSE: Trying to parse token at position 72 of type Some(Newline)
DEBUG PARSE: Skipping Newline at position 72
DEBUG PARSE: Trying to parse token at position 73 of type Some(Comment)
DEBUG SKIP: Before skip_whitespace pos=73
DEBUG SKIP: After skip_whitespace pos=73
DEBUG STMT: Parsing token with kind Comment and lexeme "; Print after user input"
DEBUG STMT: Unknown token kind: Comment
DEBUG PARSE: Failed to parse token at position 74, skipping
DEBUG PARSE: Trying to parse token at position 75 of type Some(Greater)
DEBUG SKIP: Before skip_whitespace pos=75
DEBUG SKIP: After skip_whitespace pos=75
DEBUG STMT: Parsing token with kind Greater and lexeme ">"
DEBUG STMT: Found Greater token (print operation)
DEBUG SKIP: Before skip_whitespace pos=77
DEBUG SKIP: After skip_whitespace pos=78
DEBUG PARSE: Successfully parsed Print { to_stderr: false, expr: Some(Literal("\"Counter after your input: {counter}\"")) }
DEBUG PARSE: Trying to parse token at position 80 of type Some(Newline)
DEBUG PARSE: Skipping Newline at position 80
DEBUG PARSE: Trying to parse token at position 81 of type Some(Comment)
DEBUG SKIP: Before skip_whitespace pos=81
DEBUG SKIP: After skip_whitespace pos=81
DEBUG STMT: Parsing token with kind Comment and lexeme "; Perform a calculation"
DEBUG STMT: Unknown token kind: Comment
DEBUG PARSE: Failed to parse token at position 82, skipping
DEBUG PARSE: Trying to parse token at position 83 of type Some(Star)
DEBUG SKIP: Before skip_whitespace pos=83
DEBUG SKIP: After skip_whitespace pos=83
DEBUG STMT: Parsing token with kind Star and lexeme "*"
DEBUG STMT: Found Star token
DEBUG PARSE: Successfully parsed MathOp { name: "counter", operator: Multiply, operand: Literal("2") }
DEBUG PARSE: Trying to parse token at position 91 of type Some(Newline)
DEBUG PARSE: Skipping Newline at position 91
DEBUG PARSE: Trying to parse token at position 92 of type Some(Comment)
DEBUG SKIP: Before skip_whitespace pos=92
DEBUG SKIP: After skip_whitespace pos=92
DEBUG STMT: Parsing token with kind Comment and lexeme "; Print final value"
DEBUG STMT: Unknown token kind: Comment
DEBUG PARSE: Failed to parse token at position 93, skipping
DEBUG PARSE: Trying to parse token at position 94 of type Some(Greater)
DEBUG SKIP: Before skip_whitespace pos=94
DEBUG SKIP: After skip_whitespace pos=94
DEBUG STMT: Parsing token with kind Greater and lexeme ">"
DEBUG STMT: Found Greater token (print operation)
DEBUG SKIP: Before skip_whitespace pos=96
DEBUG SKIP: After skip_whitespace pos=97
DEBUG PARSE: Successfully parsed Print { to_stderr: false, expr: Some(Literal("\"Final counter value (doubled): {counter}\"")) }
AST Tree:
├── VarDeclaration
│   ├── Name: counter
│   ├── Mutable: true
│   └── Type: Int
│       └── Value:
│           └── Literal: 0
├── Print
│   ├── to_stderr: false
│   └── Expression:
│       └── Literal: "Initial counter value: {counter}"
├── MathOp
│   ├── Name: counter
│   ├── Operator: Add
│   └── Operand:
│       └── Literal: 1
├── Print
│   ├── to_stderr: false
│   └── Expression:
│       └── Literal: "Counter after +1: {counter}"
├── MathOp
│   ├── Name: counter
│   ├── Operator: Add
│   └── Operand:
│       └── Literal: 5
├── Print
│   ├── to_stderr: false
│   └── Expression:
│       └── Literal: "Counter after +5 more: {counter}"
├── Print
│   ├── to_stderr: false
│   └── Expression:
│       └── Literal: "Enter a number to add to counter:"
├── Print
│   ├── to_stderr: false
│   └── Expression:
│       └── Identifier: counter
├── Print
│   ├── to_stderr: false
│   └── Expression:
│       └── Literal: "Counter after your input: {counter}"
├── MathOp
│   ├── Name: counter
│   ├── Operator: Multiply
│   └── Operand:
│       └── Literal: 2
└── Print
    ├── to_stderr: false
    └── Expression:
        └── Literal: "Final counter value (doubled): {counter}"
End of AST Tree
AST Tree:
├── VarDeclaration
│   ├── Name: counter
│   ├── Mutable: true
│   └── Type: Int
│       └── Value:
│           └── Literal: 0
├── Print
│   ├── to_stderr: false
│   └── Expression:
│       └── Literal: "Initial counter value: {counter}"
├── MathOp
│   ├── Name: counter
│   ├── Operator: Add
│   └── Operand:
│       └── Literal: 1
├── Print
│   ├── to_stderr: false
│   └── Expression:
│       └── Literal: "Counter after +1: {counter}"
├── MathOp
│   ├── Name: counter
│   ├── Operator: Add
│   └── Operand:
│       └── Literal: 5
├── Print
│   ├── to_stderr: false
│   └── Expression:
│       └── Literal: "Counter after +5 more: {counter}"
├── Print
│   ├── to_stderr: false
│   └── Expression:
│       └── Literal: "Enter a number to add to counter:"
├── Print
│   ├── to_stderr: false
│   └── Expression:
│       └── Identifier: counter
├── Print
│   ├── to_stderr: false
│   └── Expression:
│       └── Literal: "Counter after your input: {counter}"
├── MathOp
│   ├── Name: counter
│   ├── Operator: Multiply
│   └── Operand:
│       └── Literal: 2
└── Print
    ├── to_stderr: false
    └── Expression:
        └── Literal: "Final counter value (doubled): {counter}"
End of AST Tree
Successfully transpiled to x86_64 Assembly: runtime_interpolation.asm
