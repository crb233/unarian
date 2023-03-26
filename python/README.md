
# Unarian - Python Interpreter

This Python interpreter is intended to be a fully-featured version of the [minimal Python interpreter](../python_min/README.md) with custom classes, better error catching and messages, adjustable virtual stack depth, and modular project structure. How successfully it achieves this goal is unclear.

## Usage

The basic format of this command is as follows:
```
unarian.py [<file>] [--expr <expression>] [--depth <max-depth>] [--input] [--debug] [--compile]
```
- `[<file>]`: Optional source code file. If unspecified, no source code file will be parsed.
- `[--expr <expression>]`: Optional expression to evaluate. If unspecified, this defaults to `main`.
- `[--depth <max-depth>]`: Optional maximum stack depth of the virtual machine. If unspecified, this defaults to 10,000.
- `[--input]`: Optional flag to get input values from standard input (written as whitespace-separated decimal integers). If unspecified, this defaults to a single input of 0.
- `[--debug]`: Optional flag to turn on debugging mode, in which built-in `!` prints out the current value and `@` prints out a stack trace. Otherwise, both of these built-ins are ignored.
- `[--compile]`: Optional flag that is currently unimplemented, but may eventually compile the specified expression into a lower-level language such as C or assembly.



## Example Usage

Evaluates expression `{ - - | - + | + + }` on inputs `0`, `1`, `2`, `3`, `4`, and `5`.

```
echo 0 1 2 3 4 5 | unarian.py --expr '{ - - | - + | + + }' --input
```

Evaluates expression `{ - - | - + | + + }` with interactive input.

```
unarian.py --expr '{ - - | - + | + + }' --input
```

Evaluates `main` from `examples/collatz.un` on inputs `1`, `2`, `3`, `4`, `5`, `6`, `7`, `8`, and `9`.

```
echo 1 2 3 4 5 6 7 8 9 | unarian.py examples/collatz.un --input
```

Evaluates `if/2` from `examples/collatz.un` on inputs `0`, `1`, `2`, `3`, `4`, and `5`.

```
echo 0 1 2 3 4 5 | unarian.py examples/collatz.un --expr 'if/2' --input
```
