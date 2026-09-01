use crate::source::Span;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Builtin<'src> {
    Increment { span: Span<'src> },
    Decrement { span: Span<'src> },
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Identifier<'src> {
    span: Span<'src>
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Term<'src> {
    Builtin(Builtin<'src>),
    Identifier(Identifier<'src>),
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
    name: Identifier<'src>,
    def: Group<'src>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Library<'src> {
    span: Span<'src>,
    decs: Vec<Declaration<'src>>,
}
