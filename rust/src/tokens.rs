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

/// TODO
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    Start,
    End,
    Comment,
    OpenBrace,
    CloseBrace,
    Alternation,
    Identifier,
    Atomic(AtomicKind),
}

impl TokenKind {
    /// TODO
    #[must_use]
    fn is_opening_pair(self) -> bool {
        matches!(self, TokenKind::Start | TokenKind::OpenBrace)
    }
    
    /// TODO
    #[must_use]
    fn is_closing_pair(self) -> bool {
        matches!(self, TokenKind::End | TokenKind::CloseBrace)
    }
    
    /// TODO
    #[must_use]
    fn is_matching_pair(self, other: Self) -> bool {
        matches!((self, other),
            (TokenKind::Start, TokenKind::End) |
            (TokenKind::OpenBrace, TokenKind::CloseBrace)
        )
    }
    
    /// TODO
    #[must_use]
    fn from_span(span: &Span<'_>) -> Self {
        match span.get_str() {
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
                    TokenKind::Identifier
                }
            }
        }
    }
}

impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TokenKind::Start => write!(f, "Start"),
            TokenKind::End => write!(f, "End"),
            TokenKind::Comment => write!(f, "Comment"),
            TokenKind::OpenBrace => write!(f, "OpenBrace"),
            TokenKind::CloseBrace => write!(f, "CloseBrace"),
            TokenKind::Alternation => write!(f, "Alternation"),
            TokenKind::Identifier => write!(f, "Identifier"),
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
    fn from_span(span: Span<'src>) -> Self {
        Self::new(TokenKind::from_span(&span), span)
    }
    
    /// TODO
    #[must_use]
    pub fn get_str(&self) -> &str {
        self.span.get_str()
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
/// 
/// TODO: Maybe don't return tokens for the start and end of the stream. Then
/// also remove `TokenKind::Start` and `TokenKind::End`
impl<'src> TokenStream<'src> {
    /// TODO
    #[must_use]
    pub fn from_reader(reader: Reader<'src>) -> Self {
        let pos = reader.position().clone();
        TokenStream {
            reader,
            token: Some(Token::at_position(TokenKind::Start, pos)),
        }
    }
    
    /// TODO
    #[must_use]
    pub fn from_source(source: &'src Source) -> Self {
        TokenStream::from_reader(Reader::new(source))
    }
    
    /// TODO
    fn next_token(&mut self) -> Option<Token<'src>> {
        // no more tokens after end of input
        if self.token.is_none() || self.token.as_ref().is_some_and(|t| t.kind == TokenKind::End) {
            return None;
        }
        
        // skip whitespace
        self.reader.skip_while(|c: char| c.is_whitespace());
        
        // check for end of input
        if self.reader.is_at_end() {
            let span = Span::from_position(self.reader.position().clone());
            return Some(Token::new(TokenKind::End, span));
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
        Some(Token::new(TokenKind::from_span(&span), span))
    }
}

impl<'src> Iterator for TokenStream<'src> {
    type Item = Token<'src>;
    
    fn next(&mut self) -> Option<Self::Item> {
        let next_token = self.next_token();
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

/// TODO
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenTreeError<'src> {
    MissingTokenTree,
    UnmatchedOpeningBrace(Span<'src>),
    UnmatchedClosingBrace(Span<'src>),
}

impl<'src> TokenTree<'src> {
    // /// TODO
    // #[must_use]
    // pub fn from_token_stream(tokens: &mut TokenStream<'src>) -> Result<TokenTree<'src>, TokenTreeError<'src>> {
    //     let start = tokens.reader.position().clone();
    //     let mut contents = Vec::new();
    //     loop {
    //         match Self::next_subtree(tokens) {
    //             Ok(Some(token_tree)) => contents.push(token_tree),
    //             Ok(None) => break,
    //             Err(err) => return Err(err),
    //         }
    //     }
    //     let stop = tokens.reader.position().clone();
    //     
    //     Ok(TokenTree::Group {
    //         open: Token::at_position(TokenKind::Start, start),
    //         close: Token::at_position(TokenKind::End, stop),
    //         contents,
    //     })
    // }
    
    /// TODO
    #[must_use]
    pub fn from_reader(reader: Reader<'src>) -> Result<TokenTree<'src>, TokenTreeError<'src>> {
        let mut tokens = TokenStream::from_reader(reader);
        Self::from_token_stream(&mut tokens)
    }
    
    /// TODO
    #[must_use]
    pub fn from_source(source: &'src Source<'src>) -> Result<TokenTree<'src>, TokenTreeError<'src>> {
        let reader = Reader::new(source);
        Self::from_reader(reader)
    }
    
    /// TODO
    #[must_use]
    pub fn from_token_stream(tokens: &mut TokenStream<'src>) -> Result<TokenTree<'src>, TokenTreeError<'src>> {
        if let Some(token) = tokens.next() {
            if token.is_closing_pair() {
                Err(TokenTreeError::UnmatchedClosingBrace(token.span))
            } else if token.is_opening_pair() {
                let open = token;
                let mut contents = Vec::new();
                loop {
                    if tokens.peek().is_some_and(|tok| open.is_matching_pair(tok)) {
                        break;
                    }
                    match Self::from_token_stream(tokens) {
                        Ok(token_tree) => contents.push(token_tree),
                        Err(TokenTreeError::MissingTokenTree) => return Err(TokenTreeError::UnmatchedOpeningBrace(open.span)),
                        Err(err) => return Err(err),
                    }
                };
                let close = tokens.next().expect("peek() returned Some(_) so next should also");
                Ok(TokenTree::Group { open, close, contents })
            } else {
                Ok(TokenTree::Token(token))
            }
        } else {
            Err(TokenTreeError::MissingTokenTree)
        }
    }
    
    fn fmt_indented(&self, indent: usize, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let width: usize = 4 * indent;
        match self {
            TokenTree::Token(token) => {
                writeln!(f, "{:>width$}\"{}\" ({})", "", token.get_str(), token.kind)
            }
            TokenTree::Group { open, close, contents } => {
                writeln!(f, "{:>width$}\"{}\" ({})", "", open.get_str(), open.kind)?;
                for subtree in contents {
                    subtree.fmt_indented(indent + 1, f)?;
                }
                writeln!(f, "{:>width$}\"{}\" ({})", "", close.get_str(), close.kind)?;
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
