# Ven Engine
================

Welcome to the **Ven Engine**, an interpreted virtual machine designed to process the extremely fast **Ven** language. Below is a detailed overview of the project, its components, and its functionalities.

## Overview
----------

Ven is an interpreted language that offers high efficiency and speed, suitable for various programming tasks. The Ven Engine reads `.ven` files, tokenizes the input, parses it into an **Abstract Syntax Tree (AST)**, and then optimizes and executes the AST.

## Key Features
--------------

### Efficient Tokenization

The tokenizer in Ven is ultra-fast with minimal overhead, ensuring rapid processing of the input files.

### Robust Parsing

The parser converts tokens into an AST, handling variable declarations, input, and output operations, as well as mathematical expressions.

### AST-based Optimization

The optimizer refines the AST by removing unused variables and combining consecutive print statements for improved performance.

### Error Handling

Comprehensive error messages guide users in debugging their Ven scripts by providing detailed explanations of any issues encountered.

## Project Structure
-------------------

The project is organized into several key modules, each responsible for a specific aspect of the language processing:

### `src/main.rs`

The entry point of the application. It handles command-line arguments, reads input files, and initiates the tokenization and parsing processes.

### `src/token.rs`

Contains the tokenizer logic, converting raw input into a sequence of identifiable tokens.

### `src/parse.rs`

Implements the parsing logic, transforming tokens into an AST while handling syntax rules.

### `src/optimisers`

Houses optimization passes that enhance the AST by simplifying operations and removing redundancies.

### `src/errmsgs.rs`

Provides error messages for common issues like type mismatches and undeclared variables.

### `src/transpilers`

Provides different transpilers allowing conversion of Ven's code to other languages

## Language Syntax
----------------

Ven language features a straightforward syntax designed for clarity and efficiency:

### Variable Declarations

Use `@` for static and `@@` for mutable declarations.

| Declaration | Example |
| --- | --- |
| `@ name str "jay"` | Static declaration |
| `@@ age str 16` | Mutable declaration |

### Print Statements

To output expressions, use `>>` for stdout with support for placeholder substitution.

| Syntax | Description | Example |
| --- | --- | --- |
| `>> expression` | Print to stdout with placeholders replaced | `>> Hello {name}!` |
| `>>> expression` | Print to stderr with placeholders replaced | `>>> Error: {error_message}` |

Placeholders within curly braces `{}` are replaced with the corresponding variable values.
### Math Operations

Perform basic arithmetic with `*` for operations like addition, subtraction, multiplication, and division.

| Operation | Example |
| --- | --- |
| `* age + 2` | Addition |
| `* age - 2` | Subtraction |
| `* age * 2` | Multiplication |
| `* age / 2` | Division |

# Licensing and more
Ven is licensed under [Ven License](LICENSE.md)


Any contributions to Ven Engine are governed by the same license. By contributing to Ven Engine, you agree to the terms of the license. Any contributions are prohibited from editing the [SHELL.md](SHELL.md) file under any scenarios.
