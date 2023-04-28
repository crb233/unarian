
# Unarian - Minimal Rust Interpreter

This Rust interpreter is intended to be minimalistic with only bare-bones features. In particular, it will: raise unhelpful error messages when invalid states are discovered and sometimes accept syntactically invalid source code. However, it should always correctly evaluate valid source code without errors (ignoring exceptional circumstances like stack overflows and memory allocation errors).

## Usage

Evaluates expression `main` from Unarian file `<file>` on integer inputs `<inputs...>`:
```
unarian_min <file> <inputs...>
```

## Example Usage

Evaluates `main` from `../examples/collatz.un` on inputs `0`, `1`, `2`, `3`, `4`, and `5`:
```
unarian_min ../examples/collatz.un 0 1 2 3 4 5
```
