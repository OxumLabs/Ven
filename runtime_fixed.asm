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
│       └── Literal: "Enter a number to add to counter: "
├── Input
│       └── Name: counter
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
│       └── Literal: "Enter a number to add to counter: "
├── Input
│       └── Name: counter
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
