//================//
// String Pattern //
//================//

/// Represents a pattern in a string that can be searched for.
///
/// This was partly inspired by the currently unstable `std::str::pattern`
/// module but with a different interface and simpler functionality.
/// 
/// # Safety
/// 
/// Let `x: X` where `X` implements `Pattern` and `s: str`. Then the
/// implementation of `Pattern` for `X`must guarantee:
/// 1. `x.prefix_length_of(s).is_none_or(|n| s.is_char_boundary(n))`, i.e., if
///    a prefix length is returned, it must be at a valid character boundary in
///    the string `s`;
/// 2. `x.prefix_length_of(s).is_some() == x.matches_start(s)`, i.e., `x`
///    matches the start of `s` if and only if `x` is a prefix of `s`;
/// 3. `self.prefix_length_of(s).is_some_and(|n| n == s.len()) == x.matches(s)`,
///    i.e., `x` matches `s` if and only if the prefix length of `x` in `s` is
///    the length of `s`.
pub unsafe trait Pattern {
    /// Returns `Some(length)` giving the length of the matching pattern at the
    /// beginning of the given string, or `None` if there's no match.
    ///
    /// Note: This function returns indices into a string, so implementors must
    /// be careful to only return valid indices.
    fn prefix_length_of(self, haystack: &str) -> Option<usize>
    where Self: Sized;
    
    /// TODO
    fn matches_start(self, target: &str) -> bool
    where Self: Sized {
        self.prefix_length_of(target).is_some()
    }
    
    /// TODO
    fn matches(self, target: &str) -> bool
    where Self: Sized {
        self.prefix_length_of(target).is_some_and(|n| n == target.len())
    }
}

unsafe impl Pattern for char {
    fn prefix_length_of(self, haystack: &str) -> Option<usize> {
        match haystack.chars().next() {
            Some(c) if c == self => Some(c.len_utf8()),
            _ => None,
        }
    }
}

unsafe impl<F> Pattern for F
where F: Fn(char) -> bool {
    fn prefix_length_of(self, haystack: &str) -> Option<usize> {
        match haystack.chars().next() {
            Some(c) if self(c) => Some(c.len_utf8()),
            _ => None,
        }
    }
}

// unsafe impl<F> Pattern for &mut F
// where F: FnMut(char) -> bool {
//     fn prefix_length_of(self, haystack: &str) -> Option<usize> {
//         match haystack.chars().next() {
//             Some(c) if self(c) => Some(c.len_utf8()),
//             _ => None,
//         }
//     }
// }

unsafe impl Pattern for &str {
    fn prefix_length_of(self, haystack: &str) -> Option<usize> {
        if haystack.is_char_boundary(self.len()) && &haystack[0 .. self.len()] == self {
            Some(self.len())
        } else {
            None
        }
    }
}

// unsafe impl<F> Pattern for &F
// where F: Fn(&str) -> bool {
//     fn prefix_length_of(self, haystack: &str) -> Option<usize> {
//         for (length, _) in haystack.char_indices() {
//             let substring = &haystack[0 .. length];
//             if self(substring) {
//                 return Some(length);
//             }
//         }
//         None
//     }
// }

// #[derive(Debug, Clone, Copy)]
// pub struct Whitespace;
// 
// unsafe impl Pattern for Whitespace {
//     fn prefix_length_of(self, haystack: &str) -> Option<usize> {
//         match haystack.chars().next() {
//             Some(c) if c.is_whitespace() => Some(c.len_utf8()),
//             _ => None,
//         }
//     }
// }
// 
// #[derive(Clone, Copy)]
// pub struct Any<'a>(pub &'a [Box<dyn Pattern + 'a>]);
// 
// unsafe impl<'a> Pattern for Any<'a> {
//     fn prefix_length_of(self, haystack: &str) -> Option<usize> {
//         for &pattern in self.0 {
//             let result = pattern.prefix_length_of(haystack);
//             if result.is_some() {
//                 return result;
//             }
//         }
//         None
//     }
// }
// 
// #[derive(Clone, Copy)]
// pub struct Sequence<'a>(&'a [&'a (dyn Pattern + 'a)]);
// 
// unsafe impl<'a> Pattern for Sequence<'a> {
//     fn prefix_length_of(&self, mut haystack: &str) -> Option<usize> {
//         let mut total: usize = 0;
//         for &pattern in self.0 {
//             if let Some(n) = pattern.prefix_length_of(haystack) {
//                 total += n;
//                 haystack = &haystack[n ..];
//             } else {
//                 return None
//             }
//         }
//         Some(total)
//     }
// }
