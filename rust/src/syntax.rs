use crate::source::{IntoPeekable, PeekableIterator, Position, Span};
use crate::tokens::{AtomicKind, STRING_COMMENT_START, Token, TokenKind, TokenTree, TokenTreeStream};



/// DOC
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SyntaxTreeError<'src> {
    ExpectedCompoundFunctionIdentifier(TokenTree<'src>),
    ExpectedCompoundFunctionDefinition(TokenTree<'src>),
}



/// DOC
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Atomic<'src> {
    span: Span<'src>,
    kind: AtomicKind,
}

impl<'src> Atomic<'src> {
    fn next_from<I>(token_trees: &mut I) -> Option<Self>
    where I: PeekableIterator<Item = TokenTree<'src>> {
        if let Some(TokenTree::Token(Token { kind: TokenKind::Atomic(kind), span })) = token_trees.peek() {
            let span = span.clone();
            let kind = *kind;
            token_trees.next();
            Some(Self { span, kind })
        } else {
            None
        }
    }
}



/// DOC
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Compound<'src> {
    span: Span<'src>
}

impl<'src> Compound<'src> {
    //// DOC
    #[must_use]
    pub fn new(span: Span<'src>) -> Self {
        Self { span }
    }
    
    /// DOC
    #[must_use]
    pub fn name(&self) -> &str {
        self.span.str()
    }
    
    /// DOC
    fn next_from<I>(token_trees: &mut I) -> Option<Self>
    where I: PeekableIterator<Item = TokenTree<'src>> {
        if let Some(TokenTree::Token(Token { kind: TokenKind::Compound, span })) = token_trees.peek() {
            let span = span.clone();
            token_trees.next();
            Some(Self { span })
        } else {
            None
        }
    }
}



/// DOC
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Comments<'src> {
    comments: Vec<Span<'src>>,
}

impl<'src> Comments<'src> {
    /// DOC
    fn next_from<I>(token_trees: &mut I) -> Self
    where I: PeekableIterator<Item = TokenTree<'src>> {
        let mut comments = vec![];
        while let Some(TokenTree::Token(Token { kind: TokenKind::Comment, span })) = token_trees.peek() {
            comments.push(span.clone());
            token_trees.next();
        }
        Self { comments }
    }
    
    /// DOC
    fn skip<I>(token_trees: &mut I)
    where I: PeekableIterator<Item = TokenTree<'src>> {
        while let Some(TokenTree::Token(Token { kind: TokenKind::Comment, .. })) = token_trees.peek() {
            token_trees.next();
        }
    }
}



/// DOC
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Composition<'src> {
    span: Span<'src>,
    expressions: Vec<Expression<'src>>,
}

impl<'src> Composition<'src> {
    /// DOC
    fn new(position: Position<'src>) -> Self {
        Self {
            span: Span::from_position(position),
            expressions: vec![],
        }
    }
    
    /// DOC
    fn push(&mut self, expression: Expression<'src>) {
        self.span = Span::union([&self.span, expression.span()]);
        self.expressions.push(expression);
    }
    
    /// DOC
    fn next_from<I>(token_trees: &mut I) -> Option<Result<Self, SyntaxTreeError<'src>>>
    where I: PeekableIterator<Item = TokenTree<'src>> {
        todo!()
    }
}



/// DOC
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Alternation<'src> {
    span: Span<'src>,
    expressions: Vec<Expression<'src>>,
    operators: Vec<Span<'src>>,
}

impl<'src> Alternation<'src> {
    /// DOC
    fn new(expression: Expression<'src>) -> Self {
        Self {
            span: expression.span().clone(),
            expressions: vec![expression],
            operators: Vec::new(),
        }
    }
    
    /// DOC
    fn push(&mut self, operator: Span<'src>, expression: Expression<'src>) {
        self.span = Span::union([&self.span, &operator, expression.span()]);
        self.expressions.push(expression);
        self.operators.push(operator);
    }
    
    /// DOC
    fn next_from<I>(token_trees: &mut I) -> Option<Result<Self, SyntaxTreeError<'src>>>
    where I: PeekableIterator<Item = TokenTree<'src>> {
        todo!()
    }
}



/// DOC
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Group<'src> {
    span: Span<'src>,
    open: Span<'src>,
    close: Span<'src>,
    expression: Box<Expression<'src>>,
}

impl<'src> Group<'src> {
    fn new(open: Span<'src>, expression: Expression<'src>, close: Span<'src>) -> Self {
        let span = Span::union([&open, &expression.span(), &close]);
        Self {
            span,
            open,
            expression: Box::new(expression),
            close,
        }
    }
    
    fn from_token_tree_group(open: Token<'src>, close: Token<'src>, contents: Vec<TokenTree<'src>>) -> Self {
        todo!()
    }
    
    fn next_from<I>(token_trees: &mut I) -> Option<Result<Self, SyntaxTreeError<'src>>>
    where I: PeekableIterator<Item = TokenTree<'src>> {
        todo!()
    }
}



/// DOC
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression<'src> {
    Atomic(Atomic<'src>),
    Compound(Compound<'src>),
    Composition(Composition<'src>),
    Alternation(Alternation<'src>),
    Group(Group<'src>),
}

impl<'src> Expression<'src> {
    /// DOC
    #[must_use]
    fn span(&self) -> &Span<'src> {
        match self {
            Self::Atomic(Atomic { span, .. }) => span,
            Self::Compound(Compound { span, .. }) => span,
            Self::Composition(Composition { span, .. }) => span,
            Self::Alternation(Alternation { span, .. }) => span,
            Self::Group(Group { span, .. }) => span,
        }
    }
}



/// DOC
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Declaration<'src> {
    span: Span<'src>,
    comments: Comments<'src>,
    name: Compound<'src>,
    definition: Group<'src>,
}

impl<'src> Declaration<'src> {
    fn next_from<I>(token_trees: &mut I) -> Option<Result<Self, SyntaxTreeError<'src>>>
    where I: PeekableIterator<Item = TokenTree<'src>> {
        let comments = Comments::next_from(token_trees);
        let name = match token_trees.next() {
            Some(TokenTree::Token(Token { kind: TokenKind::Compound, span })) =>
                Compound::new(span),
            Some(tree) =>
                return Some(Err(SyntaxTreeError::ExpectedCompoundFunctionIdentifier(tree))),
            None =>
                return None,
        };
        let definition = match token_trees.next() {
            Some(TokenTree::Group { open, close, contents }) =>
                Group::from_token_tree_group(open, close, contents),
            Some(tree) =>
                return Some(Err(SyntaxTreeError::ExpectedCompoundFunctionIdentifier(tree))),
            None =>
                todo!(),
        };
        
        todo!()
    }
}



/// DOC
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Library<'src> {
    // TODO: an empty library has no span, right? Consider changing `Span`
    // definition to allow for positionless empty spans
    span: Span<'src>,
    declarations: Vec<Declaration<'src>>,
}

impl<'src> Library<'src> {
    pub fn from_token_trees(token_trees: Vec<TokenTree<'src>>) -> Result<Self, SyntaxTreeError<'src>> {
        Self::next_from(&mut token_trees.into_peekable())
    }
    
    pub fn next_from<I>(token_trees: &mut I) -> Result<Self, SyntaxTreeError<'src>>
    where I: PeekableIterator<Item = TokenTree<'src>> {
        todo!()
    }
}
