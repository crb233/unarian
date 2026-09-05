use crate::source::Span;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Atomic<'src> {
    Increment { span: Span<'src> },
    Decrement { span: Span<'src> },
    Random    { span: Span<'src> },
    Input     { span: Span<'src> },
    Output    { span: Span<'src> },
    Trace     { span: Span<'src> },
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Compound<'src> {
    span: Span<'src>
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Term<'src> {
    /// A call to an atomic function
    Atomic(Atomic<'src>),
    
    /// A call to a (possibly undefined) compound function
    Compound(Compound<'src>),
    
    /// A bracketed group
    Group(Box<Group<'src>>),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Composition<'src> {
    span: Span<'src>,
    terms: Vec<Term<'src>>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Alternation<'src> {
    span: Span<'src>,
    comps: Vec<Composition<'src>>,
    ops: Vec<Span<'src>>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Group<'src> {
    span: Span<'src>,
    open: Span<'src>,
    expr: Alternation<'src>,
    close: Span<'src>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Declaration<'src> {
    span: Span<'src>,
    name: Compound<'src>,
    def: Group<'src>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Library<'src> {
    span: Span<'src>,
    decs: Vec<Declaration<'src>>,
}

impl Library<'_> {
    fn from_token_tree_stream() {
        
    }
}
