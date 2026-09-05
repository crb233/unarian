use crate::source::{PeekableIterator, Position, Reader, Source, Span};
use crate::pattern;
use crate::pattern::Pattern;



//=========================//
// Constants and Utilities //
//=========================//

/// Comment start string
pub const STRING_COMMENT_START : &str = "#";

/// Comment stop string
pub const STRING_COMMENT_STOP  : &str = "\n";

/// Open bracketed group string
pub const STRING_OPEN_BRACE    : &str = "{";

/// Close bracketed group string
pub const STRING_CLOSE_BRACE   : &str = "}";

/// Alternation operator string
pub const STRING_ALTERNATION   : &str = "|";

/// Increment string
pub const STRING_INCREMENT     : &str = "+";

/// Decrement string
pub const STRING_DECREMENT     : &str = "-";

/// Decrement string
pub const STRING_RANDOM        : &str = "%";

/// Decrement string
pub const STRING_INPUT         : &str = "?";

/// Decrement string
pub const STRING_OUTPUT        : &str = "!";

/// Decrement string
pub const STRING_TRACE         : &str = "@";



//========//
// Tokens //
//========//

/// TODO
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AtomicKind {
    Increment,
    Decrement,
    Random,
    Input,
    Output,
    Trace,
}

impl AtomicKind {
    /// TODO
    #[must_use]
    fn from_span(span: &Span<'_>) -> Option<Self> {
        match span.str() {
            STRING_INCREMENT => Some(AtomicKind::Increment),
            STRING_DECREMENT => Some(AtomicKind::Decrement),
            STRING_RANDOM    => Some(AtomicKind::Random),
            STRING_INPUT     => Some(AtomicKind::Input),
            STRING_OUTPUT    => Some(AtomicKind::Output),
            STRING_TRACE     => Some(AtomicKind::Trace),
            string           => None,
        }
    }
}

/// TODO
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
    /// TODO
    #[must_use]
    fn is_opening_pair(self) -> bool {
        matches!(self, TokenKind::OpenBrace)
    }
    
    /// TODO
    #[must_use]
    fn is_closing_pair(self) -> bool {
        matches!(self, TokenKind::CloseBrace)
    }
    
    /// TODO
    #[must_use]
    fn is_matching_pair(self, other: Self) -> bool {
        matches!((self, other),
            (TokenKind::OpenBrace, TokenKind::CloseBrace)
        )
    }
    
    /// TODO
    #[must_use]
    fn identifier_from_span(span: &Span<'_>) -> Self {
        match AtomicKind::from_span(span) {
            Some(atomic_kind) => TokenKind::Atomic(atomic_kind),
            None => TokenKind::Compound,
        }
    }
    
    /// TODO
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



/// TODO
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token<'src> {
    pub kind: TokenKind,
    pub span: Span<'src>
}

impl<'src> Token<'src> {
    /// TODO
    #[must_use]
    fn new(kind: TokenKind, span: Span<'src>) -> Self {
        Self { kind, span }
    }
    
    /// TODO
    #[must_use]
    fn at_position(kind: TokenKind, pos: Position<'src>) -> Self {
        Self::new(kind, Span::from_position(pos))
    }
    
    /// TODO
    #[must_use]
    fn identifier_from_span(span: Span<'src>) -> Self {
        Self::new(TokenKind::identifier_from_span(&span), span)
    }
    
    /// TODO
    #[must_use]
    fn from_span(span: Span<'src>) -> Self {
        Self::new(TokenKind::from_span(&span), span)
    }
    
    /// TODO
    #[must_use]
    pub fn str(&self) -> &str {
        self.span.str()
    }
    
    /// TODO
    #[must_use]
    fn is_opening_pair(&self) -> bool {
        self.kind.is_opening_pair()
    }
    
    /// TODO
    #[must_use]
    fn is_closing_pair(&self) -> bool {
        self.kind.is_closing_pair()
    }
    
    /// TODO
    #[must_use]
    fn is_matching_pair(&self, other: &Self) -> bool {
        self.kind.is_matching_pair(other.kind)
    }
    
}



//===============//
// Token Streams //
//===============//

/// TODO
#[derive(Debug, Clone)]
pub struct TokenStream<'src> {
    reader: Reader<'src>,
    token: Option<Token<'src>>,
}

/// TODO
impl<'src> TokenStream<'src> {
    /// TODO
    #[must_use]
    pub fn new(reader: Reader<'src>) -> Self {
        let mut result = TokenStream {
            reader,
            token: None,
        };
        
        // store the first token and return
        result.token = result.next();
        result
    }
    
    /// TODO
    #[must_use]
    pub fn from_source(source: &'src Source) -> Self {
        TokenStream::new(Reader::new(source))
    }
    
    /// TODO
    fn next(&mut self) -> Option<Token<'src>> {
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
        
        // must be an identifier
        let span = self.reader.read_until(|c: char| {
            // TODO this isn't portable / compatible with STRING_COMMENT_START
            c.is_whitespace() || c == '#'
        });
        Some(Token::identifier_from_span(span))
    }
}

impl<'src> Iterator for TokenStream<'src> {
    type Item = Token<'src>;
    
    fn next(&mut self) -> Option<Self::Item> {
        // // no more tokens after end of input
        // if self.token.is_none() {
        //     return None;
        // }
        
        // read one more token, replace and return the stored one
        let next_token = self.next();
        std::mem::replace(&mut self.token, next_token)
    }
}

impl PeekableIterator for TokenStream<'_> {
    /// TODO
    fn peek(&self) -> Option<&<Self as Iterator>::Item> {
        self.token.as_ref()
    }
}



//=============//
// Token Trees //
//=============//

/// TODO
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
    /// TODO
    #[must_use]
    pub fn all(token_stream: TokenStream<'src>) -> Result<Vec<TokenTree<'src>>, TokenTreeError<'src>> {
        let mut token_tree_stream = TokenTreeStream::new(token_stream);
        let mut trees = Vec::new();
        while let Some(tree) = token_tree_stream.next()? {
            trees.push(tree);
        }
        Ok(trees)
    }
    
    /// TODO
    #[must_use]
    pub fn all_from_reader(reader: Reader<'src>) -> Result<Vec<TokenTree<'src>>, TokenTreeError<'src>> {
        Self::all(TokenStream::new(reader))
    }
    
    /// TODO
    #[must_use]
    pub fn all_from_source(source: &'src Source<'src>) -> Result<Vec<TokenTree<'src>>, TokenTreeError<'src>> {
        Self::all_from_reader(Reader::new(source))
    }
    
    /// TODO
    #[must_use]
    pub fn span(&self) -> Span<'src> {
        match self {
            TokenTree::Token(tok) => tok.span.clone(),
            TokenTree::Group { open, close, .. } =>
                Span::union(&open.span, &close.span),
        }
    }
    
    /// TODO
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

/// TODO
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenTreeError<'src> {
    UnmatchedOpeningBrace(Span<'src>),
    UnmatchedClosingBrace(Span<'src>),
}

/// TODO
/// 
/// TODO: This could easily be modified to be peekable, but is that really
/// necessary?
#[derive(Debug, Clone)]
pub struct TokenTreeStream<'src> {
    token_stream: TokenStream<'src>,
}

impl<'src> TokenTreeStream<'src> {
    /// TODO
    #[must_use]
    pub fn new(token_stream: TokenStream<'src>) -> Self {
        Self { token_stream }
    }
    
    /// TODO
    #[must_use]
    pub fn from_reader(reader: Reader<'src>) -> Self {
        Self::new(TokenStream::new(reader))
    }
    
    /// TODO
    #[must_use]
    pub fn from_source(source: &'src Source<'src>) -> Self {
        Self::from_reader(Reader::new(source))
    }
    
    /// TODO
    /// 
    /// TODO: Is there a good reason to switch to `Option<Result<TokenTree,
    /// ...>>`?
    /// 
    /// TODO: Can we make this recover gracefully from errors? What could we
    /// possibly do if braces aren't correctly matched? Hard to guess what the
    /// intent was.
    /// 
    /// TODO: As currently implemented, we can continue building token trees
    /// even after the last one failed. Is this acceptable behavior?
    pub fn next(&mut self) -> Result<Option<TokenTree<'src>>, TokenTreeError<'src>> {
        if let Some(token) = self.token_stream.next() {
            if token.is_closing_pair() {
                Err(TokenTreeError::UnmatchedClosingBrace(token.span))
            } else if token.is_opening_pair() {
                let open = token;
                let mut contents = Vec::new();
                loop {
                    if self.token_stream.peek().is_some_and(|tok| open.is_matching_pair(tok)) {
                        break;
                    }
                    match self.next() {
                        Ok(Some(token_tree)) => contents.push(token_tree),
                        Ok(None) => return Err(TokenTreeError::UnmatchedOpeningBrace(open.span)),
                        Err(err) => return Err(err),
                    }
                };
                let close = self.token_stream.next().expect("peek() returned Some(_) so next should also");
                Ok(Some(TokenTree::Group { open, close, contents }))
            } else {
                Ok(Some(TokenTree::Token(token)))
            }
        } else {
            Ok(None)
        }
    }
}
