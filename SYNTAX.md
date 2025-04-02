# Ven Language Syntax

Ven is a lightweight programming language designed for simplicity, readability, and ease of learning. This document provides a comprehensive overview of the language's syntax based on its actual implementation.

## Comments

Comments in Ven start with `;` (semicolon) and continue until the end of the line:

```
; This is a comment
```

## Variables

### Variable Declaration

Variables are declared using the `@` symbol for immutable variables or `@@` for mutable variables, followed by the variable name, type, and initial value:

```
@ name type value    ; Immutable variable
@@ name type value   ; Mutable variable
```

### Types

Ven supports the following data types:

- `i` - Integer
- `str` - String
- `f` - Float
- `c[size]` - Character with specified size

### Examples

```
@ age i 30               ; Immutable integer variable
@@ counter i 0           ; Mutable integer variable
@ message str "Hello"     ; String variable
@ pi f 3.14159           ; Float variable
@ character c[1] 'A'      ; Single character
@ buffer c[8] "BUFFER"    ; Character buffer of size 8
```

## Input/Output

### Print Statements

To print values to the console, use the `>` or `>>` operators:

```
>> expression        ; Print to stdout
>>> expression       ; Print to stderr
```

Examples:
```
>> "Hello, World!"   ; Print a string literal
>> name              ; Print the value of a variable
>> "Age: {age}"      ; String interpolation with variable reference
```

### Input Statements
you can use the `..` (double dot) operator:

```
.. variable_name     ; Also reads input into variable
```

Example:
```
> age                ; Read input into the 'age' variable
```

## Mathematical Operations

Mathematical operations on variables use the `*` symbol followed by the variable name, operator, and operand:

```
* variable_name operator operand
```

Supported operators:
- `+` (Addition)
- `-` (Subtraction)
- `*` (Multiplication)
- `/` (Division)

Examples:
```
* age + 1            ; Increment age by 1
* counter - 5        ; Decrement counter by 5
* price * 2          ; Double the price
* amount / 2         ; Halve the amount
```

## Conditional Statements

Conditional statements begin with `?` followed by a condition in parentheses and a block of code in curly braces:

```
?(condition) {
    ; code to execute if condition is true
}
```

Example:
```
?(age > 18) {
    >> "You are an adult"
}
```

### Comparison Operators

For conditions, you can use:
- `==` (Equal)
- `!=` (Not equal)
- `<` (Less than)
- `<=` (Less than or equal)
- `>` (Greater than)
- `>=` (Greater than or equal)

### Logical Operators

Conditions can be combined with:
- `&&` (Logical AND)
- `||` (Logical OR)

Example:
```
?(age > 18 && name == "Alice") {
    >> "Hello adult Alice"
}
```

## String Literals and Interpolation

String literals are enclosed in double quotes:

```
"This is a string"
```

### Variable Interpolation

Ven supports runtime variable interpolation in strings using curly braces `{}`. This allows you to embed variable values within a string:

```
"Hello, {name}! You are {age} years old."
```

When the program runs, variables within curly braces are replaced with their current values at runtime. This means:

1. Variables are evaluated at the moment the print statement executes
2. If a variable's value changes during program execution, the interpolated string will show the current value
3. You can use variables whose values are only known at runtime (like user input)

For example, this code:

```
@ name str "Alice"
@@ age i 0
>> "Hello, {name}! Your age is {age}"
> age
* age + 10
>> "In 10 years you will be {age} years old."
```

Will print the current value of `age` after it has been modified, showing the actual runtime value.

#### Key Benefits of Runtime Interpolation

1. **Dynamic Values**: Can display values that change during program execution
2. **User Input**: Can show values entered by the user during runtime
3. **Calculation Results**: Can display the results of calculations performed during execution

String interpolation works seamlessly with all of Ven's supported variable types.

#### Escaping Curly Braces

If you need to include literal curly braces in your string, you can escape them using a backslash:

```
"The variable syntax is \\{variableName\\}"
```

This will print: "The variable syntax is {variableName}"

#### Limitations

Since variable interpolation happens at compile time:

1. You cannot interpolate variables whose values are only known at runtime (like user input)
2. If a variable is modified after its initial declaration, subsequent string interpolations will still use the initial value
3. Undefined variables in string interpolation will produce compiler warnings and be replaced with `[undefined]` in the output

#### Advanced Interpolation Examples

You can use multiple variable references in a single string:

```
"Person: {firstName} {lastName}, Age: {age}, City: {city}"
```

You can mix text and variables freely:

```
"Total: {price} * {quantity} = {total} dollars"
```

Variable interpolation works with all variable types:
- Strings: `"Name: {name}"`
- Integers: `"Count: {count}"`
- Floats: `"Price: {price}"`
- Characters: `"First letter: {letter}"`

## Character Literals

Character literals are enclosed in single quotes:

```
'A'
```

## Error Handling

The Ven compiler provides detailed error messages for common issues:

- Undeclared variables
- Type mismatches
- Assignments to immutable variables
- Missing opening/closing parentheses or braces
- Invalid conditions

## Complete Example

Here's a complete example demonstrating the Ven language features:

```
; Variable declarations
@ name str "User"
@@ age i 0

; Input
>> "Please enter your age:"
> age

; Conditional statements
?(age < 18) {
    >> "Hello {name}, you are a minor."
} 

?(age >= 18) {
    >> "Hello {name}, you are an adult."
    
    ?(age > 65) {
        >> "You might be retired."
    }
}

; Math operation
* age + 10
>> "In 10 years you will be {age} years old."
```

## Language Implementation Details

Ven is implemented in Rust with a focus on performance and simplicity. The compilation process involves:

1. **Tokenization**: Dividing the source code into meaningful tokens
2. **Parsing**: Building an Abstract Syntax Tree (AST) from the tokens
3. **Optimization**: Performing multiple optimization passes on the AST
4. **Transpilation**: Converting the AST to the target language (Rust, C, LLVM IR, or x86-64 Assembly)

## Command Line Usage

```
ven --in=<file_path.ven> -t=<target> [options]
```

Where:
- `--in=<file_path.ven>`: Input file (must end with .ven)
- `-t=<target>`: Target output format (rs/rust, c, llvm, lx8664)
- `--show-msgs` or `-sm`: Show messages in a tree-like view
- `-h`, `--help`: Show help information
- `-v`, `--version`, `--ver`: Show version information 