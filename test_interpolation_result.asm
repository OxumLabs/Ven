DEBUG PARSE: Trying to parse token at position 0 of type Some(Comment)
DEBUG SKIP: Before skip_whitespace pos=0
DEBUG SKIP: After skip_whitespace pos=0
DEBUG STMT: Parsing token with kind Comment and lexeme "; Test file for string interpolation"
DEBUG STMT: Unknown token kind: Comment
DEBUG PARSE: Failed to parse token at position 1, skipping
DEBUG PARSE: Trying to parse token at position 2 of type Some(Newline)
DEBUG PARSE: Skipping Newline at position 2
DEBUG PARSE: Trying to parse token at position 3 of type Some(Comment)
DEBUG SKIP: Before skip_whitespace pos=3
DEBUG SKIP: After skip_whitespace pos=3
DEBUG STMT: Parsing token with kind Comment and lexeme "; Define some variables"
DEBUG STMT: Unknown token kind: Comment
DEBUG PARSE: Failed to parse token at position 4, skipping
DEBUG PARSE: Trying to parse token at position 5 of type Some(At)
DEBUG SKIP: Before skip_whitespace pos=5
DEBUG SKIP: After skip_whitespace pos=5
DEBUG STMT: Parsing token with kind At and lexeme "@"
DEBUG STMT: Found At token (variable declaration)
DEBUG SKIP: Before skip_whitespace pos=6
DEBUG SKIP: After skip_whitespace pos=7
DEBUG SKIP: Before skip_whitespace pos=8
DEBUG SKIP: After skip_whitespace pos=9
DEBUG SKIP: Before skip_whitespace pos=10
DEBUG SKIP: After skip_whitespace pos=11
DEBUG SKIP: Before skip_whitespace pos=11
DEBUG SKIP: After skip_whitespace pos=11
DEBUG PARSE: Successfully parsed VarDeclaration { mutable: false, name: "name", var_type: String, value: Some(Literal("Alice")) }
DEBUG PARSE: Trying to parse token at position 12 of type Some(Newline)
DEBUG PARSE: Skipping Newline at position 12
DEBUG PARSE: Trying to parse token at position 13 of type Some(At)
DEBUG SKIP: Before skip_whitespace pos=13
DEBUG SKIP: After skip_whitespace pos=13
DEBUG STMT: Parsing token with kind At and lexeme "@"
DEBUG STMT: Found At token (variable declaration)
DEBUG SKIP: Before skip_whitespace pos=14
DEBUG SKIP: After skip_whitespace pos=15
DEBUG SKIP: Before skip_whitespace pos=16
DEBUG SKIP: After skip_whitespace pos=17
DEBUG SKIP: Before skip_whitespace pos=18
DEBUG SKIP: After skip_whitespace pos=19
DEBUG SKIP: Before skip_whitespace pos=19
DEBUG SKIP: After skip_whitespace pos=19
DEBUG PARSE: Successfully parsed VarDeclaration { mutable: false, name: "age", var_type: Int, value: Some(Literal("30")) }
DEBUG PARSE: Trying to parse token at position 20 of type Some(Newline)
DEBUG PARSE: Skipping Newline at position 20
DEBUG PARSE: Trying to parse token at position 21 of type Some(At)
DEBUG SKIP: Before skip_whitespace pos=21
DEBUG SKIP: After skip_whitespace pos=21
DEBUG STMT: Parsing token with kind At and lexeme "@"
DEBUG STMT: Found At token (variable declaration)
DEBUG SKIP: Before skip_whitespace pos=22
DEBUG SKIP: After skip_whitespace pos=23
DEBUG SKIP: Before skip_whitespace pos=24
DEBUG SKIP: After skip_whitespace pos=25
DEBUG SKIP: Before skip_whitespace pos=26
DEBUG SKIP: After skip_whitespace pos=27
DEBUG SKIP: Before skip_whitespace pos=27
DEBUG SKIP: After skip_whitespace pos=27
DEBUG PARSE: Successfully parsed VarDeclaration { mutable: false, name: "price", var_type: Float, value: Some(Literal("19")) }
DEBUG PARSE: Trying to parse token at position 28 of type Some(Unknown)
DEBUG SKIP: Before skip_whitespace pos=28
DEBUG SKIP: After skip_whitespace pos=28
DEBUG STMT: Parsing token with kind Unknown and lexeme "."
DEBUG STMT: Unknown token kind: Unknown
DEBUG PARSE: Failed to parse token at position 29, skipping
DEBUG PARSE: Trying to parse token at position 30 of type Some(Newline)
DEBUG PARSE: Skipping Newline at position 30
DEBUG PARSE: Trying to parse token at position 31 of type Some(Newline)
DEBUG PARSE: Skipping Newline at position 31
DEBUG PARSE: Trying to parse token at position 32 of type Some(Comment)
DEBUG SKIP: Before skip_whitespace pos=32
DEBUG SKIP: After skip_whitespace pos=32
DEBUG STMT: Parsing token with kind Comment and lexeme "; Basic interpolation"
DEBUG STMT: Unknown token kind: Comment
DEBUG PARSE: Failed to parse token at position 33, skipping
DEBUG PARSE: Trying to parse token at position 34 of type Some(Greater)
DEBUG SKIP: Before skip_whitespace pos=34
DEBUG SKIP: After skip_whitespace pos=34
DEBUG STMT: Parsing token with kind Greater and lexeme ">"
DEBUG STMT: Found Greater token (print operation)
DEBUG SKIP: Before skip_whitespace pos=36
DEBUG SKIP: After skip_whitespace pos=37
DEBUG PARSE: Successfully parsed Print { to_stderr: false, expr: Some(Literal("\"Hello, {name}!\"")) }
DEBUG PARSE: Trying to parse token at position 39 of type Some(Newline)
DEBUG PARSE: Skipping Newline at position 39
DEBUG PARSE: Trying to parse token at position 40 of type Some(Comment)
DEBUG SKIP: Before skip_whitespace pos=40
DEBUG SKIP: After skip_whitespace pos=40
DEBUG STMT: Parsing token with kind Comment and lexeme "; Multiple variables in one string"
DEBUG STMT: Unknown token kind: Comment
DEBUG PARSE: Failed to parse token at position 41, skipping
DEBUG PARSE: Trying to parse token at position 42 of type Some(Greater)
DEBUG SKIP: Before skip_whitespace pos=42
DEBUG SKIP: After skip_whitespace pos=42
DEBUG STMT: Parsing token with kind Greater and lexeme ">"
DEBUG STMT: Found Greater token (print operation)
DEBUG SKIP: Before skip_whitespace pos=44
DEBUG SKIP: After skip_whitespace pos=45
DEBUG PARSE: Successfully parsed Print { to_stderr: false, expr: Some(Literal("\"User {name} is {age} years old\"")) }
DEBUG PARSE: Trying to parse token at position 47 of type Some(Newline)
DEBUG PARSE: Skipping Newline at position 47
DEBUG PARSE: Trying to parse token at position 48 of type Some(Comment)
DEBUG SKIP: Before skip_whitespace pos=48
DEBUG SKIP: After skip_whitespace pos=48
DEBUG STMT: Parsing token with kind Comment and lexeme "; Numeric value interpolation"
DEBUG STMT: Unknown token kind: Comment
DEBUG PARSE: Failed to parse token at position 49, skipping
DEBUG PARSE: Trying to parse token at position 50 of type Some(Greater)
DEBUG SKIP: Before skip_whitespace pos=50
DEBUG SKIP: After skip_whitespace pos=50
DEBUG STMT: Parsing token with kind Greater and lexeme ">"
DEBUG STMT: Found Greater token (print operation)
DEBUG SKIP: Before skip_whitespace pos=52
DEBUG SKIP: After skip_whitespace pos=53
DEBUG PARSE: Successfully parsed Print { to_stderr: false, expr: Some(Literal("\"The item costs ${price}\"")) }
DEBUG PARSE: Trying to parse token at position 55 of type Some(Newline)
DEBUG PARSE: Skipping Newline at position 55
DEBUG PARSE: Trying to parse token at position 56 of type Some(Comment)
DEBUG SKIP: Before skip_whitespace pos=56
DEBUG SKIP: After skip_whitespace pos=56
DEBUG STMT: Parsing token with kind Comment and lexeme "; Escaped braces"
DEBUG STMT: Unknown token kind: Comment
DEBUG PARSE: Failed to parse token at position 57, skipping
DEBUG PARSE: Trying to parse token at position 58 of type Some(Greater)
DEBUG SKIP: Before skip_whitespace pos=58
DEBUG SKIP: After skip_whitespace pos=58
DEBUG STMT: Parsing token with kind Greater and lexeme ">"
DEBUG STMT: Found Greater token (print operation)
DEBUG SKIP: Before skip_whitespace pos=60
DEBUG SKIP: After skip_whitespace pos=61
DEBUG PARSE: Successfully parsed Print { to_stderr: false, expr: Some(Literal("\"To use a variable, write \\\\{variableName\\\\}\"")) }
DEBUG PARSE: Trying to parse token at position 63 of type Some(Newline)
DEBUG PARSE: Skipping Newline at position 63
DEBUG PARSE: Trying to parse token at position 64 of type Some(Comment)
DEBUG SKIP: Before skip_whitespace pos=64
DEBUG SKIP: After skip_whitespace pos=64
DEBUG STMT: Parsing token with kind Comment and lexeme "; Complex example"
DEBUG STMT: Unknown token kind: Comment
DEBUG PARSE: Failed to parse token at position 65, skipping
DEBUG PARSE: Trying to parse token at position 66 of type Some(Greater)
DEBUG SKIP: Before skip_whitespace pos=66
DEBUG SKIP: After skip_whitespace pos=66
DEBUG STMT: Parsing token with kind Greater and lexeme ">"
DEBUG STMT: Found Greater token (print operation)
DEBUG SKIP: Before skip_whitespace pos=68
DEBUG SKIP: After skip_whitespace pos=69
DEBUG PARSE: Successfully parsed Print { to_stderr: false, expr: Some(Literal("\"Summary: {name}, age {age}, ordered product for ${price}\"")) }
DEBUG PARSE: Trying to parse token at position 71 of type Some(Newline)
DEBUG PARSE: Skipping Newline at position 71
DEBUG PARSE: Trying to parse token at position 72 of type Some(Comment)
DEBUG SKIP: Before skip_whitespace pos=72
DEBUG SKIP: After skip_whitespace pos=72
DEBUG STMT: Parsing token with kind Comment and lexeme "; Test undefined variable"
DEBUG STMT: Unknown token kind: Comment
DEBUG PARSE: Failed to parse token at position 73, skipping
DEBUG PARSE: Trying to parse token at position 74 of type Some(Greater)
DEBUG SKIP: Before skip_whitespace pos=74
DEBUG SKIP: After skip_whitespace pos=74
DEBUG STMT: Parsing token with kind Greater and lexeme ">"
DEBUG STMT: Found Greater token (print operation)
DEBUG SKIP: Before skip_whitespace pos=76
DEBUG SKIP: After skip_whitespace pos=77
DEBUG PARSE: Successfully parsed Print { to_stderr: false, expr: Some(Literal("\"This will show: {undefinedVar}\"")) }
AST Tree:
├── VarDeclaration
│   ├── Name: name
│   ├── Mutable: false
│   └── Type: String
│       └── Value:
│           └── Literal: Alice
├── VarDeclaration
│   ├── Name: age
│   ├── Mutable: false
│   └── Type: Int
│       └── Value:
│           └── Literal: 30
├── VarDeclaration
│   ├── Name: price
│   ├── Mutable: false
│   └── Type: Float
│       └── Value:
│           └── Literal: 19
├── Print
│   ├── to_stderr: false
│   └── Expression:
│       └── Literal: "Hello, {name}!"
├── Print
│   ├── to_stderr: false
│   └── Expression:
│       └── Literal: "User {name} is {age} years old"
├── Print
│   ├── to_stderr: false
│   └── Expression:
│       └── Literal: "The item costs ${price}"
├── Print
│   ├── to_stderr: false
│   └── Expression:
│       └── Literal: "To use a variable, write \\{variableName\\}"
├── Print
│   ├── to_stderr: false
│   └── Expression:
│       └── Literal: "Summary: {name}, age {age}, ordered product for ${price}"
└── Print
    ├── to_stderr: false
    └── Expression:
        └── Literal: "This will show: {undefinedVar}"
End of AST Tree
AST Tree:
├── VarDeclaration
│   ├── Name: name
│   ├── Mutable: false
│   └── Type: String
│       └── Value:
│           └── Literal: Alice
├── VarDeclaration
│   ├── Name: age
│   ├── Mutable: false
│   └── Type: Int
│       └── Value:
│           └── Literal: 30
├── VarDeclaration
│   ├── Name: price
│   ├── Mutable: false
│   └── Type: Float
│       └── Value:
│           └── Literal: 19
├── Print
│   ├── to_stderr: false
│   └── Expression:
│       └── Literal: "Hello, {name}!"
├── Print
│   ├── to_stderr: false
│   └── Expression:
│       └── Literal: "User {name} is {age} years old"
├── Print
│   ├── to_stderr: false
│   └── Expression:
│       └── Literal: "The item costs ${price}"
├── Print
│   ├── to_stderr: false
│   └── Expression:
│       └── Literal: "To use a variable, write \\{variableName\\}"
├── Print
│   ├── to_stderr: false
│   └── Expression:
│       └── Literal: "Summary: {name}, age {age}, ordered product for ${price}"
└── Print
    ├── to_stderr: false
    └── Expression:
        └── Literal: "This will show: {undefinedVar}"
End of AST Tree
Successfully transpiled to x86_64 Assembly: test_interpolation.asm
