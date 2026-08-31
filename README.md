# Unarian
Unarian (pronounced *yoo-NAIR-eein*) is an [esoteric programming language](https://en.wikipedia.org/wiki/Esoteric_programming_language) based on the concept that every expression computes a partial unary function over the natural numbers (hence the name Unarian) and these functions can only be constructed as simple combinations of existing functions.

The beauty of this language is in its simplicity. There are only two built-in functions: increment and decrement; only two ways to combine existing functions into new ones: composition and alternation; and effectively only one integer that can be accessed by running programs. Despite this simplicity, Unarian is Turing-complete and capable of representing arbitrary computable functions.

See also the [Esolangs page](https://esolangs.org/wiki/Unarian) for this language.



## This Repository
This repository contains:
- [an informal language specification](#language-specification),
- [several interesting example programs](./examples),
- [a simple VS Code extension](./vscode),
- [an involved Rust interpreter](./rust) still in progress,
- [a minimalistic Rust interpreter](./rust_min),
- [an involved Python interpreter](./python),
- and a [minimalistic Python interpreter](./python_min).

Planned additions include:
- a minimalistic C interpreter.



## Quickstart
This is a comment. It starts with any `#` and continues to the end of the line.
```
# Comments are ignored
```

This is a function declaration. It consists of an identifier (the function name), an opening bracket `{`, an expression (the function definition), and a closing bracket `}`. Any extra whitespace between these tokens (including spaces, tabs, and newlines) is ignored.
```
function_name {
	this_is_a
	function
	definition
}
```

Identifiers can contain any characters except whitespace and `#`. The tokens `|`, `{`, and `}` are special keywords that cannot be identifiers. The identifiers `+`, `-`, `%`, `?`, `!`, and `@` represent atomic functions that cannot be redefined.
```
*10 { multiply_by_10 }
/17 { divide_by_17 }
^2 { square }
```

Expressions are evaluated from left to right on their input.
```
x_squared_plus_1 { ^2 + }
x_plus_one_all_squared { + ^2 }
```

Functions can call themselves recursively.
```
infinite_loop { infinite_loop solve_p_vs_np }
```

There are two primary atomic functions: `+` and `-`. Applying `+` to input $x$ returns $x + 1$. Applying `-` to input $x$ returns $x - 1$ if $x > 0$ and fails if $x = 0$.
```
add_1 { + }
add_2 { + + }
add_5 { + + + + + }
subtract_2_or_fail { - - }
subtract_4_or_fail { - - - - }
```

There are four other atomic functions that may not included in every implementation of the language. Random `%` fails with probability $1/2$ and otherwise returns its input unchanged. Input `?` reads a single non-negative integer value from standard input and returns it. Output `!` prints its input to standard output and returns that value unchanged. Trace `@` prints the entire stack trace and returns its input unchanged.
```
fail_with_probability_1/4 { % % }
add_1_then_print_then_add_2 { + ! + + }
```

Functions can have branching execution paths. The alternation operator `|` is used to separate alternate paths.
```
do_A_or_B_or_C { A | B | C }
```

If any branch fails (by attempting to decrement from zero), then skip the rest of the current branch and evalaute the next branch on the original input.
```
subtract_2_or_add_3 { - - | + + + }
```

Empty branches act like the identity function, returning their input unchanged.
```
subtract_4_or_do_nothing { - - - - | }
subtract_up_to_3 { - - - | - - | - | }
```

A bracketed group starts with `{` and ends with `}`. Groups are evaluated as if their contents had been defined in a separate function.
```
complex {
	a { b | c } d |
	{ e | { f } } g
}

less_complex { a group_1 d | group_2 g }
group_1 { b | c }
group_2 { e | group3 }
group_3 { f }
```

The `main` function is the default entry-point for a program. It's evaluated when we run this program.
```
main { get_nth_prime }
```

Here is a simple program that divides its input by 2 if even and otherwise multiplies by 3 and adds 1.
```
# Outputs 0.
0 { - 0 | }

# Outputs 0 if the input is 0, and fails otherwise.
if=0 { { - 0 | + } - }

# Divides by 2 if divisible by 2, and fails otherwise.
if/2 { - - if/2 + | if=0 }

# Multiplies by 3.
*3 { - *3 + + + | }

# Divides by 2 if even and otherwise multiplies by 3 and adds 1.
main { if/2 | *3 + }
```



## Language Specification

### Syntax
Line comments start with `#`, continue until the end of the line, and are stripped from the source code, leaving only a newline at the end. The remainder of the code is split into *tokens*: strings of arbitrary non-whitespace characters separated from each other by whitespace. Three tokens are considered *keywords*: `{` (open brace), `}` (close brace), and `|` (alternation), and all other tokens are considered *identifiers*. Some identifiers represent *atomic functions*: `+` (increment), and `-` (decrement). Some implementations may also include `%` (random), `?` (input), `!` (output), and `@` (trace) as additional atomics. All other identifiers represent *compound functions*.

An *expression* is a (possibly empty) sequence of identifiers, alternations `|`, and bracketed groups, where a bracketed group consists of an opening brace `{`, an expression, and a closing brace `}`. For example, `- | + func { - - | } |` is an expression and `{ - { + func | } + }` is a bracketed group. A *library* is a sequence of function declarations, where a function declaration is an identifier (the function name) followed by a bracketed group (containing the function definition). Every source code file should be parsed as a library. For example, the following library defines three functions `0`, `if=0`, and `main` with corresponding definitions `- 0 |`, `{ - 0 | } -`, and `if=0 + | -`:
```
0 { - 0 | }
if=0 { { - 0 | + } - }
main { if=0 + | 0 }
```

Finally, a *program* consists of a library along with an expression, called the *entry-point*, to be evaluated in the context of that library. By default, the expression `main` is considered to be the entry-point, so any library that defines a `main` function is also a program.

### Atomic Functions
There are two primary atomics: increment `+` and decrement `-`. As their names suggest, increment adds one to its input and decrement subtracts one from its input. So evaluating `+` on input $n$ produces $n + 1$, and evaluating `-` on input $n$ for $n > 0$ produces $n - 1$. Importantly, decrement `-` *fails* on input $0$, and this failure is what makes branching execution possible.

Some implementations may add additional atomics such as: input `?`, output `!`, stack trace `@`, and random / coin flip `%`. At the moment, these are non-standard parts of the language and largely used for debugging purposes.

### Compound Functions
Compound functions are identified by their name and defined (possibly recursively) by an expression consisting of functions, compositions, alternations, and bracketed groups. To evaluate a function on input $x$, simply evaluate its definitional expression on input $x$. For example, if function `mod2` is defined by the expression `- - mod2 |`, then evaluating `mod2` on $x$ is semantically equivalent to evaluating `- - mod2 |` on $x$.

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

Evaluating an expression containing a bracketed group can be done by treating the group as a reference to a new function defined by the contents of the group. Specifically, we can evaluate `a { b | c }` by defining a new function `b|c { b | c }` and then evaluating `a b|c`. In general, for any expression containing a bracketed group `{ ... }`, define a new function `z { ... }` and replace all instances of `{ ... }` (aside from the definition of `z` itself) by `z`. For example, if `0` is a function that maps all $x$ to $0$, then the expression `- 0 | + -` also maps all $x$ to $0$. However, by adding braces, we can change this to `{ - 0 | + } -`, which maps $0$ to $0$ and fails on all other $x$ (i.e. it checks for equality with $0$).

### Implementation
To interpret or compile a Unarian program, an entry-point must be chosen. Some implementations may allow the user to specify a custom expression as the entry-point, but this is not required and should default to `main` if unspecified. It is considered undefined behavior to have references to undefined functions or multiple definitions of the same function. However, it is recommended for implementations to treat both of these cases as compilation errors.

A compiled or interpreted program is evaluated by giving it a non-negative integer input. This input is evaluated on the entry-point expression as explained above, and the resulting output, either a non-negative integer or a failure, is returned. If evaluation is non-terminating, then the program itself must not terminate. (However, it is ok and may be beneficial to detect certain non-terminating evaluations during compilation and issue warnings for them.) Input and output representations are left undefined, but it is recommended for integers to be represented in decimal and for failure to be represented by `-`. Bounds on integer inputs and outputs, as well as the behavior when these bounds are exceeded, are also left undefined, but it is recommended that implementations support integers up to at least $2^{63} - 1$ (the maximum value of a 64-bit two's complement signed integer) and produce a runtime error when exceeding their maximum value.
