# Virtual machine for Lisp like language

1. Tokenizer to convert the source code into tokens
1. Parser to convert tokens into an abstract syntax tree (AST)
1. Compiler to convert the AST to bytecode
1. Virtual machine to interpret the bytecode and output the final result
1. Disassembler to inspect the bytecode

### Features

- Arithmetic operations
- Comparison operations
- Control flow
- Variables
- Functions

### Examples

- Basic arithmetic

```
(+ 2 5)
```

- Variables

```
(var x 10)
(set x (- x 5))
```

- Control flow

```
(if (> 10 5) 1 2)

(var x 10)
(while
  (< x 20)
  (begin
    (set x (+ x 1))
  )
)
```

Run `cargo run` to see the results
