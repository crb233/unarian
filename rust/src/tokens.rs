use crate::source::{Position, Reader, Source, Span};
use crate::pattern;
use crate::pattern::Pattern;



//=========================//
// Constants and Utilities //
//=========================//

/// Comment start string
pub const CHAR_COMMENT_START   : char = '#';

/// Comment stop string
pub const CHAR_COMMENT_STOP    : char = '\n';

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
    Compound,
    Atomic(AtomicKind),
}

impl TokenKind {
    fn from_span<'src>(span: &Span<'src>) -> Self {
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
                    TokenKind::Compound
                }
            }
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
    fn new(kind: TokenKind, span: Span<'src>) -> Self {
        Self { kind, span }
    }
    
    /// TODO
    fn at_position(kind: TokenKind, pos: Position<'src>) -> Self {
        Self::new(kind, Span::from_position(pos))
    }
    
    /// TODO
    fn from_span(span: Span<'src>) -> Self {
        Self::new(TokenKind::from_span(&span), span)
    }
}



//===============//
// Token Streams //
//===============//

/// TODO
#[derive(Debug, Clone)]
pub struct TokenStream<'src> {
    token: Option<Token<'src>>,
    reader: Reader<'src>,
    next_reader: Reader<'src>,
}

impl<'src> TokenStream<'src> {
    /// TODO
    #[must_use]
    fn from_pos(reader: Reader<'src>) -> Self {
        TokenStream {
            token: Some(Token::at_position(TokenKind::Start, reader.position().clone())),
            reader: reader.clone(),
            next_reader: reader,
        }
    }
    
    /// TODO
    #[must_use]
    fn from_source(source: &'src Source) -> Self {
        TokenStream::from_pos(Reader::new(source))
    }
    
    /// TODO
    fn peek(&self) -> &Option<Token<'src>> {
        &self.token
    }
}

impl<'src> Iterator for TokenStream<'src> {
    type Item = Token<'src>;
    
    fn next(&mut self) -> Option<Self::Item> {
        // No more tokens after end of input
        if self.token.is_none() || self.token.as_ref().is_some_and(|t| t.kind == TokenKind::End) {
            self.token = None;
            return None;
        }
        
        // update last pos before updating pos
        self.reader = self.next_reader.clone();
        
        // skip whitespace
        self.next_reader.skip_while(&pattern::Whitespace);
        
        // check for end of input
        todo!();
        
        // check for a comment
        if self.next_reader.starts_with(STRING_COMMENT_START) {
            let span = self.next_reader.read_until_after(STRING_COMMENT_STOP);
            return Some(Token::new(TokenKind::Comment, span));
        }
        
        // read the identifier
        todo!();
        
        // // skip whitespace
        // self.next_position.consume_match(&*REGEX_WHITESPACE);
        // // while self.pos.peek().is_some_and(|c| c.is_ascii_whitespace()) {
        // //     self.pos.next();
        // // }
        // 
        // // try parsing the next token
        // let new_next_token = if let Some(span) = self.next_position.consume_match(&*REGEX_COMMENT) {
        //     Some(Token::new(TokenKind::Comment, span))
        // } else if let Some(span) = self.next_position.consume_match(&*REGEX_IDENTIFIER) {
        //     Some(Token::from_span(span))
        // } else {
        //     None
        // };
        
        
        
        // let new_next_token = if let Some(first) = self.pos.peek() {
        //     let start = self.pos.clone();
        //     if *first == CHAR_COMMENT_START {
        //         // parse a comment
        //         while self.pos.peek().is_some_and(|c| c!= CHAR_COMMENT_STOP) {
        //             self.pos.next();
        //         }
        //     } else {
        //         // parse a regular token
        //         while self.pos.peek().is_some_and(|c| !c.is_ascii_whitespace() && c != CHAR_COMMENT_START) {
        //             self.pos.next();
        //         }
        //     }
        //
        //     // update the next token
        //     if start != self.pos {
        //         Some(Token::from_span(Span::new(start, self.pos.clone())))
        //     } else {
        //         None
        //     }
        //
        // } else {
        //     None
        // };
        
        
        
        // // return the current token, and replace it with the next one
        // std::mem::replace(&mut self.token, new_next_token)
        todo!()
    }
}



//=============//
// Token Trees //
//=============//

/// TODO
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenTree<'src> {
    Token(Token<'src>),
    Braces {
        open: Token<'src>,
        close: Token<'src>,
        contents: Vec<TokenTree<'src>>,
    }
}

/// TODO
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenTreeError<'src> {
    UnmatchedOpeningBrace(Span<'src>),
    UnmatchedClosingBrace(Span<'src>),
}

impl<'src> TokenTree<'src> {
    /// TODO
    pub fn from_token_stream(tokens: &mut TokenStream<'src>) -> Result<TokenTree<'src>, TokenTreeError<'src>> {
        let start = tokens.reader.position().clone();
        let mut contents = Vec::new();
        loop {
            match Self::next_from_token_stream(tokens) {
                Ok(Some(token_tree)) => contents.push(token_tree),
                Ok(None) => break,
                Err(err) => return Err(err),
            }
        }
        let stop = tokens.reader.position().clone();
        
        Ok(TokenTree::Braces {
            open: Token::at_position(TokenKind::Start, start),
            close: Token::at_position(TokenKind::End, stop),
            contents,
        })
    }
    
    /// TODO
    fn next_from_token_stream(tokens: &mut TokenStream<'src>) -> Result<Option<TokenTree<'src>>, TokenTreeError<'src>> {
        if let Some(token) = tokens.next() {
            match token.kind {
                TokenKind::OpenBrace => {
                    let mut contents = Vec::new();
                    loop {
                        if tokens.peek().as_ref().is_some_and(|tok| tok.kind == TokenKind::CloseBrace) {
                            return Ok(Some(TokenTree::Braces {
                                open: token,
                                close: tokens.next().unwrap(),
                                contents,
                            }));
                        } else {
                            match Self::next_from_token_stream(tokens) {
                                Ok(Some(token_tree)) => contents.push(token_tree),
                                Ok(None) => return Err(TokenTreeError::UnmatchedOpeningBrace(token.span)),
                                err => return err,
                            }
                        }
                    }
                },
                TokenKind::CloseBrace => Err(TokenTreeError::UnmatchedClosingBrace(token.span)),
                _ => Ok(Some(TokenTree::Token(token))),
            }
        } else {
            Ok(None)
        }
    }
    
    /// TODO
    fn contents_from_token_stream(tokens: &mut TokenStream<'src>) -> Result<Vec<TokenTree<'src>>, TokenTreeError<'src>> {
        // let mut contents = Vec::new();
        // loop {
        //     if tokens.peek().as_ref().is_some_and(|tok| tok.kind == TokenKind::CloseBrace) {
        //         return Ok(contents);
        //     } else {
        //         match Self::next_from_token_stream(tokens) {
        //             Ok(Some(token_tree)) => contents.push(token_tree),
        //             Ok(None) => return Err(TokenTreeError::UnmatchedOpeningBrace(token.span)),
        //             err => return err,
        //         }
        //     }
        // }
        todo!()
    }
}
