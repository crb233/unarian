# Unarian

Unarian is an esoteric programming language based on the concept that every program computes a unary function on the natural numbers. Running a program consists of evaluating such a function on some natural number input. Programs can explicitly fail or get stuck in infinite loops, so it's more accurate to say that they compute partial functions.



### Builtins

There are two primary built-in programs: increment (`+`) and decrement (`-`). As their names suggest, increment adds one to its input and decrement subtracts one from its input. However, decrement can fail if applied to input `0` (since doing so would not produce a natural number).



### Composition

Programs can be composed by writing them in sequence: `+ + +` is the composition of `+` three times in a row. Unlike standard function composition, these programs are evaluated left to right: `a b c` evaluates as `a` then `b` then `c`.

If any program in a composition fails, then the resulting program also fails: `- - -` fails on input `0`, `1`, and `2`.

We can similarly compose user-defined programs. If `f` is defined as `+ +` and `g` is defined as `- - -`, then `f g` evaluates as `+ + - - -`, which is semantically equivalent to `-`.



### Branching

Program complexity comes from the ability to have branching execution paths. Given programs `f` and `g`, the branching program `f | g` evaluates as `f` unless `f` fails, in which case it evaluates as `g`. For example, `- | +` maps `0` to `1` because it first tries to decrement `0`, fails to do so, and then increments `0`. For all other natural numbers, `- | +` is equivalent to `-`.

The branching operator `|` is left-associative and will try evaluating branches from left to right: `a | b | c` evaluates as `a` unless `a` fails, then evaluates as `b` unless `b` fails, and finally evaluates as `c`.

Lastly, empty branches are treated as instances of the identity function: `- |` has branches `-` and ` `. It maps `0` to `0` and all other natural numbers `x` to `x - 1`.



### Syntax

```
# Basic function definition.
function_name { function_definition }

# Extra spacing doesn't matter.
example_func {
    extra
    spacing
    doesn't
    matter
}

# Function names can contain any characters except whitespace and '#'.
# Strings '+', '-', '!', and '?' are built-in and cannot be redefined.
# Strings '{', '|', and '}' have special meaning.
*10 { multiply_by_10 }
/10 { divide_by_10 }
^2 { square }

# Functions can call themselves recursively.
infinite_loop { infinite_loop solve_p_vs_np }

# There are two primary builtin programs: '+' and '-'.
# Applying '+' to input x returns x + 1
# Applying '-' to input x returns x - 1 if x > 0 and fails if x = 0
add_1 { + }
add_2 { + + }
add_3 { + + + }
subtract_2_or_fail { - - }

# There are two builtin programs used for debugging: '!' and '?'.
# Applying '!' to input x prints the value of x and returns x.
# Applying '?' to input x prints the stack trace and returns x.
print_then_add_1 { ! + }
print_stack_trace { ? }

# Programs can have branching execution paths. The special string '|' is
# used to separate different branches.
do_A_or_B_or_C { A | B | C }

# Empty branches act as the identity function.
subtract_2_or_do_nothing { - - | }

# A nested code block starts with '{' and ends with '}'. Code blocks are
# evaluated as if their contents had been defined as a separate program.
complex { a { b | c } d | { e | { f } } g }

code_1 { b | c }
code_2 { f }
code_3 { e | code_2 }
less_complex { a code_1 d | code_3 g }   # This is equivalent to 'complex'.
```



### Example

Here is an example taken from `examples/collatz.un`:

```
# Outputs 0.
0 { - 0 | }

# Fails unless input is equal to 0.
if=0 { { - 0 | + } - }

# Fails unless input is greater than 1.
if>1 { - - + + }

# Divides by 2 if divisible by 2. Fails otherwise.
if/2 { - - if/2 + | if=0 }

# Multiplies by 3.
*3 { - *3 + + + | }

# Outputs the number of collatz steps required to reach 1.
collatz { if>1 { if/2 | *3 + } collatz + | - }

main { collatz }
```



### Usage

Evaluates expression `{ - - | - + | + + }` on inputs `0`, `1`, `2`, `3`, `4`, and `5`.

```
echo 0 1 2 3 4 5 | ./unarian.py --expr '{ - - | - + | + + }' --input
```

Evaluates expression `{ - - | - + | + + }` with interactive input.

```
./unarian.py --expr '{ - - | - + | + + }' --input
```

Evaluates `main` from `examples/collatz.un` on inputs `1`, `2`, `3`, `4`, `5`, `6`, `7`, `8`, and `9`.

```
echo 1 2 3 4 5 6 7 8 9 | ./unarian.py examples/collatz.un --input
```

Evaluates `if/2` from `examples/collatz.un` on inputs `0`, `1`, `2`, `3`, `4`, and `5`.

```
echo 0 1 2 3 4 5 | ./unarian.py examples/collatz.un --expr 'if/2' --input
```
