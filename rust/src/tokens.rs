use crate::source::{PeekableIterator, Position, Reader, Source, Span};
use crate::pattern::{self, Any, Whitespace};
use crate::pattern::Pattern;



//=========================//
// Constants and Utilities //
//=========================//

/// Comment start
pub const STRING_COMMENT_START : &str = "#";

/// Comment stop
pub const STRING_COMMENT_STOP  : &str = "\n";

/// Open bracketed group symbol
pub const STRING_OPEN_BRACE    : &str = "{";

/// Close bracketed group symbol
pub const STRING_CLOSE_BRACE   : &str = "}";

/// Alternation operator symbol
pub const STRING_ALTERNATION   : &str = "|";

/// Increment atomic function
pub const STRING_INCREMENT     : &str = "+";

/// Decrement atomic function
pub const STRING_DECREMENT     : &str = "-";

/// Random atomic function
pub const STRING_RANDOM        : &str = "%";

/// Input atomic function
pub const STRING_INPUT         : &str = "?";

/// Output atomic function
pub const STRING_OUTPUT        : &str = "!";

/// Trace atomic function
pub const STRING_TRACE         : &str = "@";



//========//
// Tokens //
//========//

/// DOC
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AtomicKind {
    Increment,
    Decrement,
    Random,
    Input,
    Output,
    Trace,
}

/// DOC
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    Comment,
    OpenBrace,
    CloseBrace,
    Alternation,
    Compound,
    Atomic(AtomicKind),
}

impl TokenKind {
    /// DOC
    #[must_use]
    fn is_opening_pair(self) -> bool {
        matches!(self, TokenKind::OpenBrace)
    }
    
    /// DOC
    #[must_use]
    fn is_closing_pair(self) -> bool {
        matches!(self, TokenKind::CloseBrace)
    }
    
    /// DOC
    #[must_use]
    fn is_matching_pair(self, other: Self) -> bool {
        matches!((self, other),
            (TokenKind::OpenBrace, TokenKind::CloseBrace)
        )
    }
    
    /// DOC
    #[must_use]
    fn from_span(span: &Span<'_>) -> Self {
        match span.str() {
            STRING_OPEN_BRACE  => TokenKind::OpenBrace,
            STRING_CLOSE_BRACE => TokenKind::CloseBrace,
            STRING_ALTERNATION => TokenKind::Alternation,
            STRING_INCREMENT   => TokenKind::Atomic(AtomicKind::Increment),
            STRING_DECREMENT   => TokenKind::Atomic(AtomicKind::Decrement),
            STRING_RANDOM      => TokenKind::Atomic(AtomicKind::Random),
            STRING_INPUT       => TokenKind::Atomic(AtomicKind::Input),
            STRING_OUTPUT      => TokenKind::Atomic(AtomicKind::Output),
            STRING_TRACE       => TokenKind::Atomic(AtomicKind::Trace),
            string => {
                if STRING_COMMENT_START.matches_start(string) {
                    TokenKind::Comment
                } else {
                    TokenKind::Compound
                }
            }
        }
    }
}

impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TokenKind::Comment => write!(f, "Comment"),
            TokenKind::OpenBrace => write!(f, "OpenBrace"),
            TokenKind::CloseBrace => write!(f, "CloseBrace"),
            TokenKind::Alternation => write!(f, "Alternation"),
            TokenKind::Compound => write!(f, "Compound"),
            TokenKind::Atomic(AtomicKind::Increment) => write!(f, "Atomic(Increment)"),
            TokenKind::Atomic(AtomicKind::Decrement) => write!(f, "Atomic(Decrement)"),
            TokenKind::Atomic(AtomicKind::Random) => write!(f, "Atomic(Random)"),
            TokenKind::Atomic(AtomicKind::Input) => write!(f, "Atomic(Input)"),
            TokenKind::Atomic(AtomicKind::Output) => write!(f, "Atomic(Output)"),
            TokenKind::Atomic(AtomicKind::Trace) => write!(f, "Atomic(Trace)"),
        }
    }
}



/// DOC
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token<'src> {
    pub kind: TokenKind,
    pub span: Span<'src>
}

impl<'src> Token<'src> {
    /// DOC
    #[must_use]
    fn new(kind: TokenKind, span: Span<'src>) -> Self {
        Self { kind, span }
    }
    
    /// DOC
    #[must_use]
    fn at_position(kind: TokenKind, pos: Position<'src>) -> Self {
        Self::new(kind, Span::from_position(pos))
    }
    
    /// DOC
    #[must_use]
    fn from_span(span: Span<'src>) -> Self {
        Self::new(TokenKind::from_span(&span), span)
    }
    
    /// DOC
    #[must_use]
    pub fn str(&self) -> &str {
        self.span.str()
    }
    
    /// DOC
    #[must_use]
    fn is_opening_pair(&self) -> bool {
        self.kind.is_opening_pair()
    }
    
    /// DOC
    #[must_use]
    fn is_closing_pair(&self) -> bool {
        self.kind.is_closing_pair()
    }
    
    /// DOC
    #[must_use]
    fn is_matching_pair(&self, other: &Self) -> bool {
        self.kind.is_matching_pair(other.kind)
    }
    
}



//===============//
// Token Streams //
//===============//

/// DOC
#[derive(Debug, Clone)]
pub struct TokenStream<'src> {
    reader: Reader<'src>,
    token: Option<Token<'src>>,
}

/// DOC
impl<'src> TokenStream<'src> {
    /// DOC
    #[must_use]
    pub fn new(reader: Reader<'src>) -> Self {
        let mut result = TokenStream {
            reader,
            token: None,
        };
        
        // store the first token and return
        result.token = result.next_token();
        result
    }
    
    /// DOC
    #[must_use]
    pub fn from_source(source: &'src Source) -> Self {
        TokenStream::new(Reader::new(source))
    }
    
    /// DOC
    fn next_token(&mut self) -> Option<Token<'src>> {
        // skip whitespace
        self.reader.skip_while(|c: char| c.is_whitespace());
        
        // check for end of input
        if self.reader.is_at_end() {
            let span = Span::from_position(self.reader.position().clone());
            return None;
        }
        
        // check for a comment
        if self.reader.starts_with(STRING_COMMENT_START) {
            let span = self.reader.read_until(STRING_COMMENT_STOP);
            return Some(Token::new(TokenKind::Comment, span));
        }
        
        // must be a "regular" token (symbol or identifier)
        let span = self.reader.read_until(Any(&[
            &Whitespace,
            &STRING_COMMENT_START,
        ]));
        Some(Token::from_span(span))
    }
}

impl<'src> Iterator for TokenStream<'src> {
    type Item = Token<'src>;
    
    fn next(&mut self) -> Option<Self::Item> {
        // try reading one more token, replace and return the stored one
        let next_token = self.next_token();
        std::mem::replace(&mut self.token, next_token)
    }
}

impl PeekableIterator for TokenStream<'_> {
    /// DOC
    fn peek(&self) -> Option<&<Self as Iterator>::Item> {
        self.token.as_ref()
    }
}



//=============//
// Token Trees //
//=============//

/// DOC
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenTree<'src> {
    Token(Token<'src>),
    Group {
        open: Token<'src>,
        close: Token<'src>,
        contents: Vec<TokenTree<'src>>,
    },
}

impl<'src> TokenTree<'src> {
    /// DOC
    #[must_use]
    pub fn all(token_stream: TokenStream<'src>) -> Result<Vec<TokenTree<'src>>, TokenTreeError<'src>> {
        TokenTreeStream::new(token_stream).collect()
    }
    
    /// DOC
    #[must_use]
    pub fn all_from_reader(reader: Reader<'src>) -> Result<Vec<TokenTree<'src>>, TokenTreeError<'src>> {
        Self::all(TokenStream::new(reader))
    }
    
    /// DOC
    #[must_use]
    pub fn all_from_source(source: &'src Source<'src>) -> Result<Vec<TokenTree<'src>>, TokenTreeError<'src>> {
        Self::all_from_reader(Reader::new(source))
    }
    
    /// DOC
    #[must_use]
    pub fn span(&self) -> Span<'src> {
        match self {
            TokenTree::Token(tok) => tok.span.clone(),
            TokenTree::Group { open, close, .. } =>
                Span::union(&open.span, &close.span),
        }
    }
    
    /// DOC
    #[must_use]
    pub fn fmt_indented(&self, indent: usize, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        const INDENT_AMOUNT: usize = 4;
        match self {
            TokenTree::Token(token) => {
                writeln!(f, "{:>indent$}\"{}\" ({})", "", token.str(), token.kind)
            }
            TokenTree::Group { open, close, contents } => {
                writeln!(f, "{:>indent$}\"{}\" ({})", "", open.str(), open.kind)?;
                for subtree in contents {
                    subtree.fmt_indented(indent + INDENT_AMOUNT, f)?;
                }
                writeln!(f, "{:>indent$}\"{}\" ({})", "", close.str(), close.kind)?;
                Ok(())
            }
        }
    }
}

impl std::fmt::Display for TokenTree<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
       self.fmt_indented(0, f)
    }
}

/// DOC
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenTreeError<'src> {
    UnmatchedOpeningBrace(Span<'src>),
    UnmatchedClosingBrace(Span<'src>),
}

/// DOC
/// 
/// TODO: This could easily be modified to be peekable, but is that really
/// necessary?
#[derive(Debug, Clone)]
pub struct TokenTreeStream<'src> {
    token_stream: TokenStream<'src>,
}

impl<'src> TokenTreeStream<'src> {
    /// DOC
    #[must_use]
    pub fn new(token_stream: TokenStream<'src>) -> Self {
        Self { token_stream }
    }
    
    /// DOC
    #[must_use]
    pub fn from_reader(reader: Reader<'src>) -> Self {
        Self::new(TokenStream::new(reader))
    }
    
    /// DOC
    #[must_use]
    pub fn from_source(source: &'src Source<'src>) -> Self {
        Self::from_reader(Reader::new(source))
    }
}

impl<'src> Iterator for TokenTreeStream<'src> {
    type Item = Result<TokenTree<'src>, TokenTreeError<'src>>;
    
    /// DOC
    /// 
    /// TODO: Can we make this recover gracefully from errors? What could we
    /// possibly do if braces aren't correctly matched? Hard to guess what the
    /// intent was.
    /// 
    /// TODO: As currently implemented, we can continue building token trees
    /// even after the last one failed. Is this acceptable behavior?
    fn next(&mut self) -> Option<Self::Item> {
        let token = self.token_stream.next()?;
        if token.is_closing_pair() {
            return Some(Err(TokenTreeError::UnmatchedClosingBrace(token.span)));
        } else if !token.is_opening_pair() {
            return Some(Ok(TokenTree::Token(token)));
        }
        
        // we have an opening pair, so try building a group
        let open = token;
        let mut contents = Vec::new();
        
        // while we haven't found the matching closing pair, try adding a
        // subtree to the group
        while self.token_stream.peek().is_none_or(|tok| !open.is_matching_pair(tok)) {
            match self.next() {
                Some(Ok(token_tree)) => contents.push(token_tree),
                Some(Err(err)) => return Some(Err(err)),
                None => return Some(Err(TokenTreeError::UnmatchedOpeningBrace(open.span))),
            }
        };
        let close = self.token_stream.next().expect("peek() returned Some(_) so next() should also");
        Some(Ok(TokenTree::Group { open, close, contents }))
    }
}
