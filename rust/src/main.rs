#![allow(unused, clippy::double_must_use)]

use crate::tokens::{TokenTree, TokenTreeStream};

mod source;
mod tokens;
mod syntax;
mod pattern;
mod message;



fn main() {
    let code = "
        # Returns 0.
        0 { - 0 | }
        
        # Returns n if n = 0 and fails otherwise.
        if=0 { { - 0 | + } - }
        
        # Returns n if n = 1 and fails otherwise.
        if=1 { - if=0 + }
        
        # Returns 3 * n.
        *3 { - *3 + + + | }
        
        # Returns n / 2 if n % 2 = 0 and fails otherwise.
        if/2 { - if/2 + + | if=0 }
        
        # Implements the Collatz map.
        collatz { if/2 | *3 + }
        
        # Implements the Collatz counting function.
        collatz-count { if=1 | collatz collatz-count }
        
        main { collatz-count }
    ";
    
    let source = source::Source::new("<test-code>", code);
    let trees = TokenTree::all_from_source(&source).unwrap();
    for tree in trees {
        println!("{tree}");
    }
}
