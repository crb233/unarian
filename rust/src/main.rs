#![allow(unused)]

mod source;
mod tokens;
mod syntax;
mod pattern;

trait X {
    
}

impl X for &str {}

impl X for i64 {}

fn main() {
    println!("Hello, world!");
    let a: &str = "hello";
    let b: i64 = -13248;
    let z: [&dyn X; 2] = [&a, &b];
}



//============================//
// Error and Warning Messages //
//============================//

//=== Errors ===//

// Error: unmatched opening brace
//     path/to/file.un at 1:1
//       ╻
//     1 ┃ main {
//       ┃      ^
//       ╹
// Error: unmatched closing brace
//     path/to/file.un at 1:1
//       ╻
//     1 ┃ func{ - | + }
//       ┃             ^
//       ╹
// Error: expected a function identifier; found a keyword
//     path/to/file.un at 1:1
//       ╻
//     1 ┃ | {
//       ┃ ^
//       ╹
// Error: expected a function identifier; found an atomic function
//     path/to/file.un at 1:1
//       ╻
//     1 ┃ + {
//       ┃ ^
//       ╹
// Error: expected a function identifier; found a group
//     path/to/file.un at 1:1
//       ╻
//     1 ┃ { - - +
//       ┃ ^^^^^^^
//     2 ┃     | - +
//       ┃ ^^^^^^^^^
//     3 ┃     | }
//       ┃ ^^^^^^^
//       ╹
// Error: expected a bracketed function definition "{ ... }" for the function "my_func"
//     path/to/file.un at 1:1
//       ╻
//     1 ┃ my_func ( )
//       ┃         ^
//       ╹
// Error: incompatible definitions of the function "do_something"
//     path/to/file.un at 1:1
//       ╻
//     1 ┃ do_something { - }
//       ┃ ^^^^^^^^^^^^^^^^^^
//       ╹
//     path/to/another/file.un at 1:1
//       ╻
//     1 ┃ do_something { - + }
//       ┃ ^^^^^^^^^^^^^^^^^^^^
//       ╹
// Error: reference to an undefined function "main"
//     path/to/file.un at 1:1
//       ╻
//     1 ┃ { main ! }
//       ┃   ^^^^
//       ╹



//=== Warnings ===//

// Warning: identifiers should not contain the keyword '|'
//     path/to/file.un at 1:1
//       ╻
//     1 ┃ main { do|a|thing }
//       ┃        ^^^^^^^^^^
//       ╹
// Warning: identifiers should not contain the keyword '{'
//     path/to/file.un at 1:1
//       ╻
//     1 ┃ main { run{ }
//       ┃        ^^^^
//       ╹
// Warning: identifiers should not contain the keyword '}'
//     path/to/file.un at 1:1
//       ╻
//     1 ┃ main { run} }
//       ┃        ^^^^
//       ╹
// Warning: unnecessary braces around an empty expression
//     path/to/file.un at 1:1
//       ╻
//     1 ┃ main { - { } + }
//       ┃          ^^^
//       ╹
// Warning: unnecessary braces around a composition
//     path/to/file.un at 1:1
//       ╻
//     1 ┃ main { - { - + } + }
//       ┃          ^^^^^^^
//       ╹
// Warning: unnecessary braces around an alternation
//     path/to/file.un at 1:1
//       ╻
//     1 ┃ main { { - - | - + } | + }
//       ┃        ^^^^^^^^^^^^^
//       ╹
// Warning: unused function "func"
//     path/to/file.un at 1:1
//       ╻
//     1 ┃ func { + + }
//       ┃ ^^^^
//       ╹
// Warning: multiple equivalent definitions of the function "func"
//     path/to/file.un at 1:1
//       ╻
//     1 ┃ func { + + }
//       ┃ ^^^^^^^^^^^^
//       ╹
//     path/to/file.un at 2:1
//       ╻
//     2 ┃ func { + { + - + } }
//       ┃ ^^^^^^^^^^^^^^^^^^^^
//       ╹
// Warning: ineffective expression
//     path/to/file.un at 1:1
//       ╻
//     1 ┃ main { + - }
//       ┃        ^^^
//       ╹
// Warning: ineffective expression
//     path/to/file.un at 1:1
//       ╻
//     1 ┃ main { + *4 - - /2 - /2 }
//       ┃        ^^^^^^^^^^^^^^^^
//       ╹
// Warning: ineffective expression
//     path/to/file.un at 1:1
//       ╻
//     1 ┃ main { try-thing | - + | }
//       ┃                    ^^^^^
//       ╹
// Warning: unreachable code following a never-failing expression
//     path/to/file.un at 1:1
//       ╻
//     1 ┃ main { - + | | + + }
//       ┃                ^^^
//       ╹
// Warning: unreachable code following a never-failing expression
//     path/to/file.un at 1:1
//       ╻
//     1 ┃ main { - + fail-if-zero | fail-if-nonzero | + + }
//       ┃                                             ^^^
//       ╹
// Warning: unreachable code following an always-failing expression
//     path/to/file.un at 1:1
//       ╻
//     1 ┃ main { 0 - - + | }
//       ┃            ^^^
//       ╹
// Warning: unreachable code following an always-failing expression
//     path/to/file.un at 1:1
//       ╻
//     1 ┃ main { fail-if-zero | fail-if-zero + + | }
//       ┃                                    ^^^
//       ╹
// Warning: unreachable code following a non-terminating expression
//     path/to/file.un at 1:1
//       ╻
//     1 ┃ main { loop - + | }
//       ┃             ^^^^^
//       ╹
// Warning: unreachable code following a non-terminating expression
//     path/to/file.un at 1:1
//       ╻
//     1 ┃ main { - + loop-if-nonzero | loop-if-zero - + | }
//       ┃                                           ^^^^^
//       ╹
// Warning: non-effectful non-terminating function "main" on input 0
//     path/to/file.un at 1:1
//       ╻
//     1 ┃ main { fail-if-nonzero main }
//       ┃ ^^^^
//       ╹
