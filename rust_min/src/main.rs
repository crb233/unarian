type Library = std::collections::HashMap<String, Alternation>;
type Alternation = Vec<Composition>;
type Composition = Vec<Term>;

enum Term {
    Increment,
    Decrement,
    Group(Alternation),
    Function(String),
}

type Value = Option<u64>;

use std::slice::Iter;
enum Frame<'a> {
    Term(&'a Term),
    Composition(Iter<'a, Term>),
    Alternation(Iter<'a, Composition>),
    AlternationBackup(Iter<'a, Composition>, Value),
}

fn tokenize(src: &str) -> impl Iterator<Item = &str> {
    src.lines()
        .flat_map(|line| line.split('#').next())
        .flat_map(|line| line.split_whitespace())
}

fn parse_group<'src, Tokens>(tokens: &mut Tokens) -> Result<Alternation, &'static str>
where Tokens: Iterator<Item = &'src str> {
    let mut alt = Alternation::new();
    let mut comp = Composition::new();
    while let Some(token) = tokens.next() {
        match token {
            "}" => {
                alt.push(comp);
                return Ok(alt);
            }
            "|" => {
                alt.push(comp);
                comp = Composition::new();
            }
            "{" => comp.push(Term::Group(parse_group(tokens)?)),
            "+" => comp.push(Term::Increment),
            "-" => comp.push(Term::Decrement),
            _ => comp.push(Term::Function(String::from(token))),
        }
    }
    Err("unmatched '{'")
}

fn parse<'src, Tokens>(tokens: &mut Tokens) -> Result<Library, &'static str>
where Tokens: Iterator<Item = &'src str> {
    let mut lib = Library::new();
    while let Some(name) = tokens.next() {
        tokens.next();
        lib.insert(String::from(name), parse_group(tokens)?);
    }
    Ok(lib)
}

fn evaluate(lib: &Library, alt: &Alternation, mut value: Value) -> Result<Value, &'static str> {
    let mut stack = vec![Frame::Alternation(alt.iter())];
    while let Some(frame) = stack.pop() {
        match frame {
            Frame::Term(Term::Increment) => value = value.and_then(|x| x.checked_add(1)),
            Frame::Term(Term::Decrement) => value = value.and_then(|x| x.checked_sub(1)),
            Frame::Term(Term::Group(alt)) => stack.push(Frame::Alternation(alt.iter())),
            Frame::Term(Term::Function(name)) => {
                let alt = lib.get(name).ok_or("undefined function")?;
                stack.push(Frame::Alternation(alt.iter()));
            }
            Frame::Composition(mut comp) => {
                if let Some(term) = comp.next() {
                    stack.push(Frame::Composition(comp));
                    stack.push(Frame::Term(term));
                }
            }
            Frame::Alternation(alt) => {
                stack.push(Frame::AlternationBackup(alt, value));
                value = None;
            }
            Frame::AlternationBackup(mut alt, backup @ Some(_)) => {
                if let (Some(comp), None) = (alt.next(), value) {
                    stack.push(Frame::AlternationBackup(alt, backup));
                    stack.push(Frame::Composition(comp.iter()));
                    value = backup;
                }
            }
            Frame::AlternationBackup(_, None) => {}
        }
    }
    Ok(value)
}

fn main() -> Result<(), &'static str> {
    let mut args = std::env::args().skip(1);
    let lib = {
        let path = args.next().ok_or("must specify source code file")?;
        let source = std::fs::read_to_string(path).map_err(|_| "unable to open source code file")?;
        let mut tokens = tokenize(source.as_str());
        parse(&mut tokens)?
    };
    
    let expr = vec![vec![Term::Function(String::from("main"))]];
    for arg in args {
        let value = arg.parse::<u64>().map_err(|_| "unable to parse integer")?;
        println!("{:?} -> {:?}", value, evaluate(&lib, &expr, Some(value)));
    }
    Ok(())
}
