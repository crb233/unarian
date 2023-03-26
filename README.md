# Unarian

Unarian is an esoteric programming language based on the concept that every operation computes a unary function over the natural numbers (hence the name Unarian). Running a Unarian program consists of evaluating such a function on a natural number input. These operations can explicitly fail or get stuck in infinite loops, so it's more accurate to say that they compute partial unary functions.

The beauty of this language is in its simplicity. There are only two built-in functions: increment and decrement, and only two ways to combine existing functions into new ones: composition and alternation. Despite this simplicity, Unarian is capable of representing arbitrary computable functions.



## This Repository

This repository contains:
- [a short language specification](#language-specification),
- [several example programs](./examples)
- [an involved Python interpreter](./python)
- and a [minimalistic Python interpreter](./python_min).

Planned additions include:
- a simple Visual Studio Code extension for syntax highlighting,
- a minimalistic C interpreter,
- and a fully-featured Rust interpreter including a custom bytecode format.



## Language Specification

### Syntax

Line comments start with `#` and are stripped from the source code before parsing. The remainder of the code is split into tokens: strings of arbitrary non-whitespace characters separated from each other by whitespace. Three tokens are considered reserved keywords: `{`, `}`, and `|`. A few additional tokens have built-in behavior: `+`, and `-` (and sometimes `?`, `!`, and `@`). All other tokens are valid function identifiers.

A Unarian program consists of a sequence of function definitions. If it defines a `main` function, this is considered the entry-point for the program. Otherwise it is considered a library rather than a standalone program. A function definition consists of an identifier, an opening brace `{`, the content of the function, and finally a matching closing brace `}`. Within function definitions, braces are used to group expressions together.



### Built-ins

There are two primary built-in functions: increment `+` and decrement `-`. As their names suggest, increment adds one to its input and decrement subtracts one from its input. However, decrement can fail if applied to input $0$ (since doing so would not produce a natural number). Some implementations (including this one) may add additional built-in functions such as: input `?`, output `!`, and stack trace `@`. At the moment, these are non-standard parts of the language and largely used for debugging purposes.



### Composition

Composition is one method of combining existing functions to create a new one. It is an associative binary operator over Unarian functions. Syntactically, the composition of functions `f` and `g` is written as `f g`.

Evaluating a composition on input $x$ consists of evaluating each function from left to right on the output of the previous function. The result of the composition is the result of the last function to be evaluated. For example, if `^2` is a function that squares its input, then `^2 +` maps $x$ to $x^2 + 1$ and `+ ^2` maps $x$ to $(x + 1)^2$. Observe that this is similar to standard function composition in mathematics, except with the order of evaluation reversed. Significantly, if any function in a composition fails, then the composite function as a whole also fails. For example, `- - -` fails on input $0$, $1$, or $2$, and returns $n - 3$ on input $n > 2$.

Finally, an empty composition is treated as the identity function, which turns out to be the identity element of function composition. Syntactically, an empty composition can be written as an empty group `{ }`.



### Alternation

Alternation (formerly called branching) is the second method of combining existing functions. It is an associative binary operator over Unarian functions. Syntactically, the alternation of functions `f` and `g` is written as `f | g`. This operator has a lower precedence than composition, so `f g | h` is equivalent to `{ f g } | h` and `f | g h` is equivalent to `f | { g h }`.

Evaluating an alternation on input $x$ consists of evaluating each function from left to right on input $x$ if and only if all previous functions failed. The result of the alternation is the result of the last function to be evaluated. For example, if `%2` is a function that fails on odd inputs and leaves all others unchanged, then `%2 + | -` maps $2x$ to $2x + 1$ and $2x + 1$ to $2x$. Syntactically, an empty 'branch' of an alternation is considered to be an instance of the identity function. For example, `- | ` is semantically equivalent to `- | { }`, where `{ }` is the identity function.

Finally, since there is no way to represent them syntactically, we don't define the behavior of empty alternations (although it seems logical to define an empty alternation as a function that fails on all input, since this is the identity element of function alternation).



### Learn by Example

```
# This is a comment.

# This is a basic function definition.
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

# There are two primary builtin functions: '+' and '-'.
# Applying '+' to input x returns x + 1
# Applying '-' to input x returns x - 1 if x > 0 and fails if x = 0
add_1 { + }
add_2 { + + }
add_3 { + + + }
subtract_2_or_fail { - - }

# There are three builtin functions used for debugging: '?', '!', and '@'.
# Applying '?' reads a value from standard input and returns it.
# Applying '!' to input x prints x to standard input and returns x.
# Applying '@' to input x prints the stack trace and returns x.
print_then_add_1 { ! + }
print_stack_trace { @ }

# Functions can have branching execution paths. The special string '|' is
# used to separate different branches.
do_A_or_B_or_C { A | B | C }

# Empty branches act as the identity function.
subtract_2_or_do_nothing { - - | }

# A nested code block starts with '{' and ends with '}'. Code blocks are
# evaluated as if their contents had been defined as a separate function.
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
