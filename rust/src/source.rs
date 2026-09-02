use std::borrow::{Borrow, Cow};
use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::path::Path;
use std::str::Chars;

use crate::pattern::Pattern;



//================//
// Misc Utilities //
//================//

/// Returns the number of characters in the standard decimal representation of
/// an arbitrary usize.
fn usize_str_width(x: usize) -> usize {
    1 + x.checked_ilog10().unwrap_or(0) as usize
}

/// This is almost identical to `std::iter::Peekable`, except that we don't
/// need a mutable reference to peek at the next item. The advantage of
/// `std::iter::Peekable` is that it's lazy and won't compute the next item
/// until absolutely necessary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Peekable<I>
where I: Iterator {
    current: Option<I::Item>,
    iterator: I,
}

impl<I> Peekable<I>
where I: Iterator {
    /// Creates and returns a new `Peekable` from the given iterator.
    pub fn new(mut iterator: I) -> Self {
        let current = iterator.next();
        Self { current, iterator }
    }
    
    /// Peeks at the next element in the iterator without advancing it.
    pub fn peek(&self) -> &Option<I::Item> {
        &self.current
    }
}

/// `Peekable` is itself an iterator.
impl<I> Iterator for Peekable<I>
where I: Iterator {
    type Item = I::Item;
    
    fn next(&mut self) -> Option<Self::Item> {
        std::mem::replace(&mut self.current, self.iterator.next())
    }
}

/// This trait adds a method that transforms any iterator into a `Peekable`.
pub trait IntoPeekable: Sized + Iterator {
    fn into_peekable(self) -> Peekable<Self>;
}

/// Implement for all iterators.
impl<I> IntoPeekable for I
where I: Iterator {
    fn into_peekable(self) -> Peekable<Self> {
        Peekable::new(self)
    }
}



//=============//
// Source Code //
//=============//

/// Represents a single named source of code.
/// 
/// TODO: Consider renaming Text or Code.
#[derive(Clone, PartialEq, Eq)]
pub struct Source<'s> {
    /// The name of the source. For example, could be `"<input>"` to represent
    /// user input or `"./filename.txt"` to represent a file.
    name: String,
    
    /// The text of the source code.
    text: Cow<'s, str>,
}

impl<'s> Debug for Source<'s> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Source {{ name: \"{}\" }}", self.name)
    }
}

impl<'s> Source<'s> {
    /// Create a new Source with the specified name and text.
    #[must_use]
    pub fn new<N, T>(name: N, text: T) -> Self
    where N: Into<String>, T: Into<Cow<'s, str>> {
        Source {
            name: name.into(),
            text: text.into(),
        }
    }
    
    /// Attempt to create a new Source from the file at a specified path.
    pub fn from_file<P: AsRef<Path>>(path: &P) -> std::io::Result<Self> {
        let name = path.as_ref().to_string_lossy().into_owned();
        let text = std::fs::read_to_string(path)?;
        Ok(Source::new(name, text))
    }
    
    /// TODO
    unsafe fn get_slice(&self, start: usize, end: usize) -> &str {
        let text: &str = self.text.borrow();
        &text[start .. end]
    }
}

impl<'a> Display for Source<'a> {
    /// Display only the name of the Source, not its text.
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}



//==================//
// Source Positions //
//==================//

/// Represents a character position within the text of a Source.
#[derive(Clone)]
pub struct Position<'src> {
    /// The source text that this position refers to.
    src: &'src Source<'src>,
    
    /// Index (in bytes / u8s, starting at 0) of the start of the current character.
    char_byte: usize,
    
    /// Index (in bytes / u8s, starting at 0) of the start of the current line.
    line_byte: usize,
    
    /// Index (in chars, starting at 0) of the current character.
    char_index: usize,
    
    /// Index (in lines, starting at 0) of the current line.
    line_index: usize,
    
    /// Index (in chars, starting at 0) of the current column.
    col_index: usize,
}

impl<'src> Position<'src> {
    /// Creates a new position at the start of the specified Source.
    #[must_use]
    pub fn new(src: &'src Source<'src>) -> Self {
        Self {
            src,
            char_byte: 0,
            line_byte: 0,
            char_index: 0,
            line_index: 0,
            col_index: 0,
        }
    }
    
    /// Moves this position back to the start of the source.
    pub fn move_to_start(&mut self) {
        self.char_byte = 0;
        self.line_byte = 0;
        self.char_index = 0;
        self.line_index = 0;
        self.col_index = 0;
    }
    
    /// Returns the index of the current character.
    pub fn get_char_index(&self) -> usize {
        self.char_index
    }
    
    /// Returns the index of the current line.
    pub fn get_line_index(&self) -> usize {
        self.line_index
    }
    
    /// Returns the index of the current column.
    pub fn get_col_index(&self) -> usize {
        self.col_index
    }
    
    /// Returns the human-readable number of the current line.
    pub fn get_line_number(&self) -> usize {
        self.line_index
    }
    
    /// Returns the human-readable number of the current column.
    pub fn get_col_number(&self) -> usize {
        self.col_index
    }
    
    /// Given the next character in the source, increments this position to
    /// point after that character. This function is not public and should only
    /// be called internally to avoid inconsistent state.
    fn advance_by_char(&mut self, c: char) {
        self.char_byte += c.len_utf8();
        self.char_index += 1;
        if c == '\n' {
            self.line_byte = self.char_byte;
            self.line_index += 1;
            self.col_index = 0;
        } else {
            self.col_index += 1;
        }
    }
    
    /// Given some next substring in the source, increments this position to
    /// point after that substring. This function is not public and should only
    /// be called internally to avoid inconsistent state.
    fn advance_by_str(&mut self, s: &str) {
        for c in s.chars() {
            self.advance_by_char(c);
        }
    }
}

/// This must be implemented manually because `Chars` doesn't implement `PartialEq`
impl<'src> PartialEq for Position<'src> {
    /// Two positions compare equal if and only if they are from the same source
    /// and they have the same byte index.
    fn eq(&self, other: &Self) -> bool {
        self.src == other.src && self.char_byte == other.char_byte
    }
}

/// This must be implemented manually because `Chars` doesn't implement `Eq`
impl<'src> Eq for Position<'src> { }

/// This must be implemented manually because `Chars` doesn't implement `PartialOrd`
impl<'src> PartialOrd for Position<'src> {
    /// Returns the natural ordering between two positions based on their
    /// distance from the start of the source.
    ///
    /// # Panics
    ///
    /// - Panics if the two positions come from different sources.
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        assert_eq!(self.src, other.src, "Can only compare positions from the same source.");
        Some(self.char_byte.cmp(&other.char_byte))
    }
}

/// This must be implemented manually because `Chars` doesn't implement `Ord`
impl<'src> Ord for Position<'src> {
    /// Returns the natural ordering between two positions based on their
    /// distance from the start of the source.
    ///
    /// # Panics
    ///
    /// - Panics if the two positions come from different sources.
    fn cmp(&self, other: &Self) -> Ordering {
        assert_eq!(self.src, other.src, "Can only compare positions from the same source.");
        self.char_byte.cmp(&other.char_byte)
    }
}

impl<'src> Debug for Position<'src> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Position {{ src: \"{}\", line_index: {}, col_index: {} }}", self.src.name, self.line_index, self.col_index)
    }
}

impl<'src> Display for Position<'src> {
    /// Display the current position in a human-readable format, including the
    /// name of the source text, and the position's line and column numbers.
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{} at {}:{}", self.src.name, self.line_index + 1, self.col_index + 1)
    }
}



//==============//
// Source Spans //
//==============//

/// Represents a substring within the text of a Source.
#[derive(Clone, PartialEq, Eq)]
pub struct Span<'src> {
    /// The Source within which the substring lives
    src: &'src Source<'src>,
    
    /// The starting position of the substring. This is inclusive, so whatever
    /// character it refers to is included in the substring.
    start: Position<'src>,
    
    /// The ending position of the substring. This is exclusive, so whatever
    /// character it refers to is excluded from the substring.
    end: Position<'src>,
}

impl<'src> Span<'src> {
    /// Returns a new span between two positions in the same source.
    ///
    /// # Panics
    ///
    /// - Panics if the start and end positions are from different sources.
    /// - Panics if the start position comes after the end position.
    #[must_use]
    pub fn new(start: Position<'src>, end: Position<'src>) -> Self {
        assert_eq!(start.src, end.src, "A span can only be formed with positions from the same source.");
        assert!(start <= end, "A span can only be formed with a start position before the end position.");
        Span { src: start.src, start, end }
    }
    
    /// Returns a new span at a single position in a source.
    #[must_use]
    pub fn from_position(pos: Position<'src>) -> Self {
        Span { src: pos.src, start: pos.clone(), end: pos }
    }
    
    /// Returns the union of two spans, i.e. the smallest span containing them both.
    ///
    /// Note that the union of an arbitrary span `a` with an empty (zero length)
    /// span `b` won't necessarily return a span equivalent to `a`. This is
    /// because of how we define containment for spans (see `Span::contains`).
    ///
    /// # Panics
    ///
    /// - Panics if the two spans come from different sources.
    #[must_use]
    pub fn union(a: &Span<'src>, b: &Span<'src>) -> Span<'src> {
        assert_eq!(a.src, b.src, "A union can only be formed with spans from the same source.");
        let start = if a.start <= b.start { a.start.clone() } else { b.start.clone() };
        let end = if a.end >= b.end { a.end.clone() } else { b.end.clone() };
        Span::new(start, end)
    }
    
    /// Checks whether this span contains another.
    ///
    /// # Panics
    ///
    /// - Panics if the two spans come from different sources.
    #[must_use]
    pub fn contains(&self, other: &Span<'src>) -> bool {
        assert_eq!(self.src, other.src, "Containment can only be checked for spans from the same source.");
        return self.start <= other.start && self.end >= other.end;
    }
    
    /// Returns the number of bytes contained in this span, when each character
    /// is represented in UTF-8 (the default for Rust strings).
    #[must_use]
    pub fn num_bytes(&self) -> usize {
        self.end.char_byte - self.start.char_byte
    }
    
    /// Returns the number of characters contained in this span.
    ///
    /// Note that this function counts Unicode characters and doesn't account
    /// for groups of such characters that form a single grapheme.
    ///
    /// TODO Maybe fix this
    #[must_use]
    pub fn num_chars(&self) -> usize {
        self.end.char_index - self.start.char_index
    }
    
    /// Returns the substring that this span represents.
    #[must_use]
    pub fn get_str(&self) -> &'src str {
        &self.src.text[self.start.char_byte .. self.end.char_byte]
    }
    
    /// Returns a vector of strings for each line of the span.
    #[must_use]
    pub fn get_lines(&self) -> Vec<(usize, &'src str)> {
        (self.start.line_index .. self.end.line_index + 1)
            .zip(self.src.text[self.start.line_byte ..].lines())
            .collect()
    }
    
    /// Returns a vector of strings that represent, in a human-readable format,
    /// each line of the span within the context of the source text.
    ///
    /// Note that this function assumes that every character takes up the same
    /// width on screen. The alignment will not be correct for non-monospace
    /// fonts, graphemes formed from multiple characters, or characters that
    /// represent non-standard width graphemes.
    /// 
    /// TODO: Properly handle graphemes and unicode character width. See the
    /// crates `unicode_width` and `unicode_segmentation` for more.
    /// 
    /// TODO: Does it make sense for spans to ignore comments? This would
    /// increase the complexity of this code since it would have to detect
    /// comments.
    #[must_use]
    pub fn get_formatted_lines(&self) -> Vec<String> {
        let lines = self.get_lines();
        let last_line = lines.last().expect("at least one line");
        let (last_row, last_text) = last_line;
        let width = usize_str_width(*last_row);
        
        // Handle the special case of just one line
        if lines.len() == 1 {
            let mut strings = Vec::new();
            let col = self.start.col_index;
            let num = std::cmp::max(self.end.col_index - self.start.col_index, 1);
            strings.push(format!("{:>width$} ╻", ""));
            strings.push(format!("{:>width$} ┃ {}", last_row + 1, last_text));
            strings.push(format!("{:>width$} ┃ {: >col$}{:^>num$}", "", "", ""));
            strings.push(format!("{:>width$} ╹", ""));
            return strings;
        }
        
        // Init and prefix
        let mut strings = Vec::new();
        strings.push(format!("{:>width$} ╻", ""));
        
        // First line
        if let Some((line_ind, text)) = lines.first() {
            let col = self.start.col_index;
            let num = text.len() - self.start.col_index;
            strings.push(format!("{:>width$} ┃ {}", line_ind + 1, text));
            strings.push(format!("{:>width$} ┃ {: >col$}{:^>num$}", "", "", ""));
        }
        
        // Middle lines
        for (line_ind, text) in &lines[1 .. lines.len() - 1] {
            let num = text.len();
            strings.push(format!("{:>width$} ┃ {}", line_ind + 1, text));
            strings.push(format!("{:>width$} ┃ {:^>num$}", "", ""));
        }
        
        // Last line
        {
            let num = self.end.col_index;
            strings.push(format!("{:>width$} ┃ {}", last_row + 1, last_text));
            strings.push(format!("{:>width$} ┃ {:^>num$}", "", ""));
        }
        
        // Suffix and return
        strings.push(format!("{:>width$} ╹", ""));
        strings
    }
    
    /// Alternative form of <Self as Display>::fmt that formats this span with a
    /// specified indentation.
    pub fn fmt_indented(&self, f: &mut Formatter, indent: usize) -> std::fmt::Result {
        for line in self.get_formatted_lines() {
            write!(f, "{:>indent$}{}", "", line)?;
        }
        Ok(())
    }
}

impl<'s> Debug for Span<'s> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Span {{ start: {}, end: {} }}", self.start, self.end)
    }
}

impl<'src> Display for Span<'src> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{} from {} to {}", self.src, self.start, self.end)
    }
}



//===============//
// Source Reader //
//===============//

/// TODO
#[derive(Debug, Clone)]
pub struct Reader<'src> {
    /// The source text to read.
    src: &'src Source<'src>,
    
    /// Peekable character iterator to read from.
    chars: Peekable<Chars<'src>>,
    
    /// The current position of this Reader within the source text.
    pos: Position<'src>,
}

impl<'src> Reader<'src> {
    /// Creates a new Reader at the start of the source.
    pub fn new(src: &'src Source<'src>) -> Self {
        Reader {
            src,
            chars: src.text.chars().into_peekable(),
            pos: Position::new(&src),
        }
    }
    
    /// Returns the current position of this Reader.
    pub fn position(&self) -> &Position<'src> {
        &self.pos
    }
    
    /// Move back to the start of the source.
    pub fn move_to_start(&mut self) {
        self.chars = self.src.text.chars().into_peekable();
        self.pos.move_to_start();
    }
    
    /// Moves forward the specified number of bytes. The resulting position must
    /// be on a valid character boundary.
    fn move_forward(&mut self, bytes: usize) {
        let char_byte = self.pos.char_byte + bytes;
        assert!(self.src.text.is_char_boundary(char_byte));
        while self.pos.char_byte < char_byte {
            self.next();
        }
    }
    
    /// Returns the character at the current position if it exists.
    pub fn peek(&self) -> &Option<char> {
        self.chars.peek()
    }
    
    /// Increments this position to point to the next character in the source,
    /// and returns the current character if it exists.
    pub fn next(&mut self) -> Option<char> {
        if let Some(c) = self.chars.next() {
            self.pos.advance_by_char(c);
            Some(c)
        } else {
            None
        }
    }
    
    /// TODO
    pub fn is_at_start(&self) -> bool {
        self.pos.char_byte == 0
    }
    
    /// TODO
    pub fn is_at_end(&self) -> bool {
        self.peek().is_none()
    }
    
    /// TODO
    fn remainder(&self) -> &'src str {
        unsafe { self.src.get_slice(self.pos.char_byte, self.src.text.len()) }
    }
    
    /// TODO
    pub fn starts_with<P: Pattern>(&mut self, pattern: P) -> bool {
        pattern.prefix_length_of(self.remainder()).is_some()
    }
    
    /// TODO
    pub fn skip_once<P: Pattern>(&mut self, pattern: P) {
        if let Some(n) = pattern.prefix_length_of(self.remainder()) {
            self.move_forward(n);
        }
    }
    
    /// TODO
    pub fn skip_while<P: Pattern + Copy>(&mut self, pattern: P) {
        while let Some(n) = pattern.prefix_length_of(self.remainder()) {
            self.move_forward(n);
        }
    }
    
    /// TODO
    pub fn skip_until<P: Pattern + Copy>(&mut self, pattern: P) {
        while pattern.prefix_length_of(self.remainder()).is_none() {
            if self.next().is_none() {
                break;
            }
        }
    }
    
    /// TODO
    /// 
    /// TODO should this fail if we don't eventually match the pattern?
    pub fn skip_until_after<P: Pattern + Copy>(&mut self, pattern: P) {
        loop {
            if let Some(n) = pattern.prefix_length_of(self.remainder()) {
                self.move_forward(n);
                break;
            }
            if self.next().is_none() {
                break;
            }
        }
    }
    
    /// TODO
    pub fn read_once<P: Pattern>(&mut self, pattern: P) -> Option<Span<'src>> {
        if let Some(n) = pattern.prefix_length_of(self.remainder()) {
            let start = self.pos.clone();
            self.move_forward(n);
            return Some(Span::new(start, self.pos.clone()));
        }
        None
    }
    
    /// TODO
    pub fn read_while<P: Pattern>(&mut self, pattern: P) -> Vec<Span<'src>> {
        let mut matches = Vec::new();
        if let Some(n) = pattern.prefix_length_of(self.remainder()) {
            let start = self.pos.clone();
            self.move_forward(n);
            matches.push(Span::new(start, self.pos.clone()));
        }
        matches
    }
    
    /// TODO
    pub fn read_until<P: Pattern + Copy>(&mut self, pattern: P) -> Span<'src> {
        let start = self.pos.clone();
        self.skip_until(pattern);
        Span::new(start, self.pos.clone())
    }
    
    /// TODO
    /// 
    /// TODO should this fail if we don't eventually match the pattern?
    pub fn read_until_after<P: Pattern + Copy>(&mut self, pattern: P) -> Span<'src> {
        let start = self.pos.clone();
        self.skip_until_after(pattern);
        Span::new(start, self.pos.clone())
    }
}
