# Unarian

Unarian (pronounced _yoo-NAIR-eein_) is an esoteric programming language based on the concept that every operation computes a unary function over the natural numbers (hence the name Unarian). Running a Unarian program consists of evaluating such a function on a natural number input. These operations can explicitly fail or get stuck in infinite loops, so it's more accurate to say that they compute partial unary functions.

The beauty of this language is in its simplicity. There are only two built-in functions: increment and decrement, and only two ways to combine existing functions into new ones: composition and alternation. Despite this simplicity, Unarian is capable of representing arbitrary computable functions.

See also the [Esolangs page](https://esolangs.org/wiki/Unarian) for this language.



## This Repository

This repository contains:
- [a short language specification](#language-specification),
- [several example programs](./examples),
- [a simple VS Code extension](./vscode),
- [an involved Python interpreter](./python),
- and a [minimalistic Python interpreter](./python_min).

Planned additions include:
- a minimalistic C interpreter,
- and a fully-featured Rust interpreter including a custom bytecode format.



## Language Specification

### Syntax

Line comments start with `#` and are stripped from the source code before parsing. The remainder of the code is split into tokens: strings of arbitrary non-whitespace characters separated from each other by whitespace. Three tokens are considered special keywords: `{` (open brace), `}` (close brace), and `|` (alternation). A few additional tokens represent built-ins: `+` (increment), and `-` (decrement). Some implementations may also include `?` (input), `!` (output), and `@` (stack trace) as additional builtins. All other tokens are considered valid function identifiers.

A Unarian expression is a sequence of alternations `|`, built-ins, identifiers, and bracketed groups, where a bracketed group consists of an opening brace `{`, an expression, and a closing brace `}`. For example, `- | + func { - - | } |` is an expression and `{ - { + func | } + }` is a bracketed group. A Unarian program consists of a sequence of function declarations, where a function declaration is an identifier (the function name) followed by a bracketed group (containing the function definition). For example, `f { - - | + }` declares a function named `f` defined by the expression `- - | +`, and the following program defines three functions `0`, `if=0`, and `main`:
```
0 { - 0 | }
if=0 { { - 0 | + } - }
main { if=0 + | 0 }
```



### Built-ins

There are two primary built-ins: increment `+` and decrement `-`. As their names suggest, increment adds one to its input and decrement subtracts one from its input. However, decrement can fail if applied to input $0$ (since doing so would not produce a natural number).

Some implementations may add additional built-ins such as: input `?`, output `!`, and stack trace `@`. At the moment, these are non-standard parts of the language and largely used for debugging purposes.



### Functions

Functions are identified by their name and defined (possibly recursively) by an expression consisting of built-ins, functions, compositions, and alternations. To evaluate a function on input $x$, simply evaluate its definitional expression on input $x$. For example, if function `mod2` is defined by the expression `- - mod2 |`, then evaluating `mod2` on $x$ is semantically equivalent to evaluating `- - mod2 |` on $x$.



### Composition

Composition is one method of combining existing functions to create new ones. It is an associative binary operator over Unarian functions that is comparable to sequential execution (e.g. `a; b`) in imperative languages. Syntactically, the composition of functions `f` and `g` is written as `f g`.

Evaluating a composition on input $x$ consists of evaluating each function from left to right on the output of the previous function. The result of the composition is the result of the last function to be evaluated. For example, if `^2` is a function that squares its input, then `^2 +` maps $x$ to $x^2 + 1$ and `+ ^2` maps $x$ to $(x + 1)^2$. Observe that this is similar to standard function composition in mathematics, except with the order of evaluation reversed. Significantly, if any function in a composition fails, then the composite function as a whole also fails. For example, `- - -` fails on input $0$, $1$, and $2$, and returns $n - 3$ on input $n > 2$.

Finally, an empty composition is treated as the identity function, which turns out to be the identity element of function composition. Syntactically, an empty composition can be written as an empty group `{ }` or an empty expression ` `.



### Alternation

Alternation (formerly called branching) is the second method of combining existing functions. It is an associative binary operator over Unarian functions that is comparable to conditional control flow (e.g. `if c then a else b`) in imperative languages. Syntactically, the alternation of functions `f` and `g` is written as `f | g`. This operator has a lower precedence than composition, so `f g | h` is interpreted as the alternation of `f g` and `h` (written `{ f g } | h`), and `f | g h` is interpreted as the alternation of `f` and `g h` (written `f | { g h }`).

Evaluating an alternation on input $x$ consists of evaluating each function from left to right on input $x$ if and only if all previous functions failed. The result of the alternation is the result of the last function to be evaluated. For example, if `%2` is a function that fails on odd inputs and leaves all others unchanged, then `%2 + | -` maps $2x$ to $2x + 1$ and $2x + 1$ to $2x$ (i.e. it toggles the last bit in a binary number). Syntactically, an empty 'branch' of an alternation is considered to be an empty composition. For example, `- | ` is semantically equivalent to both `- | { }` and `- | id`, where `id` is an identity function.

Finally, since there is no way to represent them syntactically, we don't define the behavior of empty alternations (although it seems logical to define an empty alternation as a function that fails on all input, since this is the identity element of function alternation).



### Grouping

Bracketed groups within an expression, which are surrounded by braces and can be nested, allow for the formation of expressions that don't follow normal precedence rules. While `a b | c` is interpreted as the alternation of `a b` and `c`, the expression `a { b | c }` is interpreted as the composition of `a` and `b | c`.

Evaluating an exression containing a bracketed group can be done by treating the group as a reference to a new function defined by the contents of the group. Specifically, we can evaluate `a { b | c }` by defining a new function `b|c { b | c }` and then evaluating `a b|c`. In general, for any expression containing a bracketed group `{ ... }`, define a new function `z { ... }` and replace all instances of `{ ... }` (aside from the definition of `z` itself) by `z`. For example, if `0` is a function that maps all $x$ to $0$, then the expression `- 0 | + -` also maps all $x$ to $0$. However, by adding braces, we can change this to `{ - 0 | + } -`, which maps $0$ to $0$ and fails on all other $x$ (i.e. it checks for equality with $0$).



### Evaluation

When interpreting or compiling a Unarian program, an expression must be chosen as the entry-point. This entry-point defaults to `main`. Some implementations may allow the user to specify a custom expression as the entry-point, but this is not required. It is considered undefined behavior to have references to undefined functions or multiple definitions of the same function. However, it is recommended for implementations to treat both of these cases as compilation errors.

Finally, a compiled or interpreted program is evaluated by giving it a non-negative integer input. This input is evaluated on the entry-point expression as explained above, and the resulting output, either a non-negative integer or a failure, is returned. Input and output representations are left undefined, but it is recommended for integers to be represented in decimal and for failure to be represented by `-`. Bounds on integer inputs and outputs, as well as the behavior when these bounds are exceeded, are also left undefined, but it is recommended that implementations support integers up to at least $2^{63} - 1$ and produce a runtime error when exceeding their maximum value.



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
# Tokens '{', '|', and '}' are special keywords and cannot be function names.
# Tokens '+', '-', '?', '!', and '@' are built-in and cannot be redefined.
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

# Functions can have branching execution paths. The special token '|', called
# alternation, is used to separate alternate paths.
do_A_or_B_or_C { A | B | C }

# Empty alternates act as the identity function.
subtract_2_or_do_nothing { - - | }

# A bracketed group starts with '{' and ends with '}'. Such groups are
# evaluated as if their contents had been defined in a separate function.
complex { a { b | c } d | { e | { f } } g }

code_1 { b | c }
code_2 { f }
code_3 { e | code_2 }
less_complex { a code_1 d | code_3 g }   # This is equivalent to 'complex'.

# The main function is the default entry-point for a program. It's evaluated
# when we run this program.
main { get_nth_prime }
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
