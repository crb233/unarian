use crate::source::{Source, Position, Span, Reader};

// use lazy_static::lazy_static;
// use regex::Regex;



//=========================//
// Constants and Utilities //
//=========================//

/// Whitespace characters
pub const CHARS_WHITESPACE     : &str = " \t\n\r";

/// Comment start string
pub const CHAR_COMMENT_START   : char = '#';

/// Comment stop string
pub const CHAR_COMMENT_STOP    : char = '\n';

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

// lazy_static!{
//     static ref REGEX_WHITESPACE : Regex = Regex::new(r"\s+").unwrap();
//     static ref REGEX_COMMENT    : Regex = Regex::new(r"#.*$").unwrap();
//     static ref REGEX_IDENTIFIER : Regex = Regex::new(r"[^\s#]+").unwrap();
//     // static ref REGEX_COMMENT_START : Regex = Regex::new(r"#").unwrap();
//     // static ref REGEX_COMMENT_STOP  : Regex = Regex::new(r"\n").unwrap();
// }




//========//
// Tokens //
//========//

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    FileStart,
    FileStop,
    Comment,
    OpenBrace,
    CloseBrace,
    Alternation,
    Increment,
    Decrement,
    Identifier,
}

impl TokenKind {
    fn from_span<'src>(span: &Span<'src>) -> Self {
        match span.get_str() {
            STRING_OPEN_BRACE  => TokenKind::OpenBrace,
            STRING_CLOSE_BRACE => TokenKind::CloseBrace,
            STRING_ALTERNATION => TokenKind::Alternation,
            STRING_INCREMENT   => TokenKind::Increment,
            STRING_DECREMENT   => TokenKind::Decrement,
            string => {
                if string.starts_with(CHAR_COMMENT_START) {
                    TokenKind::Comment
                } else {
                    TokenKind::Identifier
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token<'src> {
    pub kind: TokenKind,
    pub span: Span<'src>
}

impl<'src> Token<'src> {
    fn new(kind: TokenKind, span: Span<'src>) -> Self {
        Self { kind, span }
    }
    
    fn at_position(kind: TokenKind, pos: Position<'src>) -> Self {
        Self::new(kind, Span::from_position(pos))
    }
    
    fn from_span(span: Span<'src>) -> Self {
        Self::new(TokenKind::from_span(&span), span)
    }
}



//===============//
// Token Streams //
//===============//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenStream<'src> {
    token: Option<Token<'src>>,
    position: Position<'src>,
    next_position: Position<'src>,
}

impl<'src> TokenStream<'src> {
    #[must_use]
    fn from_pos(position: Position<'src>) -> Self {
        TokenStream {
            token: Some(Token::at_position(TokenKind::FileStart, position.clone())),
            position: position.clone(),
            next_position: position,
        }
    }
    
    #[must_use]
    fn from_source(source: &'src Source) -> Self {
        TokenStream::from_pos(Position::new(source))
    }
}

impl<'src> TokenStream<'src> {
    fn get_pos(&self) -> &Position<'src> {
        &self.position
    }
    
    fn peek(&self) -> &Option<Token<'src>> {
        &self.token
    }
}

impl<'src> Iterator for TokenStream<'src> {
    type Item = Token<'src>;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.token.is_none() {
            return None;
        }
        
        // update last pos before updating pos
        self.position = self.next_position.clone();
        
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenTree<'src> {
    Token(Token<'src>),
    Braces {
        open: Token<'src>,
        close: Token<'src>,
        contents: Vec<TokenTree<'src>>,
    }
}

pub enum TokenTreeError<'src> {
    UnmatchedOpeningBrace(Span<'src>),
    UnmatchedClosingBrace(Span<'src>),
}

impl<'src> TokenTree<'src> {
    pub fn from_token_stream(tokens: &mut TokenStream<'src>) -> Result<TokenTree<'src>, TokenTreeError<'src>> {
        let start = tokens.position.clone();
        let mut contents = Vec::new();
        loop {
            match Self::next_from_token_stream(tokens) {
                Ok(Some(token_tree)) => contents.push(token_tree),
                Ok(None) => break,
                Err(err) => return Err(err),
            }
        }
        let stop = tokens.position.clone();
        
        Ok(TokenTree::Braces {
            open: Token::at_position(TokenKind::FileStart, start),
            close: Token::at_position(TokenKind::FileStop, stop),
            contents,
        })
    }
    
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
