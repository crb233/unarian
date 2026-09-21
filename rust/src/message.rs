use crate::source::Span;

/// DOC
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MessageLevel {
    /// DOC
    Suggestion,
    
    /// DOC
    Warning,
    
    /// DOC
    Error,
}

/// DOC
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Message<'src> {
    //========//
    // Errors //
    //========//
    
    /// DOC
    UnmatchedOpeningBrace {
        open: Span<'src>,
    },
    
    /// DOC
    UnmatchedClosingBrace {
        close: Span<'src>,
    },
    
    /// DOC
    ExpectedCompoundFunctionIdentifier {
        found: Span<'src>,
        kind: String,
    },
    
    /// DOC
    ExpectedCompoundFunctionDefinition {
        name: String,
        found: Span<'src>,
        kind: String,
    },
    
    /// DOC
    IncompatibleDefinitions {
        function: String,
        definitions: Vec<Span<'src>>,
    },
    
    /// DOC
    UndefinedFunctionReference {
        reference: Span<'src>,
        suggestions: Vec<String>,
    },
    
    //==========//
    // Warnings //
    //==========//
    
    /// DOC
    IdentifierContainsKeyword {
        idenfifier: Span<'src>,
        keywords: Vec<String>,
    },
    
    /// DOC
    UnnecessaryBraces {
        expression: Span<'src>,
        kind: String,
    },
    
    /// DOC
    UnusedFunction {
        name: String,
        declaration: Span<'src>,
    },
    
    /// DOC
    EquivalentDefinitions {
        name: String,
        declarations: Vec<Span<'src>>,
    },
    
    /// DOC
    /// 
    /// TODO: maybe this should be `IneffectiveCode`, since what's ineffective
    /// cannot always be considered an expression (e.g., if it includes an
    /// alternation operator)
    IneffectiveExpression {
        expression: Span<'src>,
    },
    
    /// DOC
    UnreachableCode {
        code: Span<'src>,
        reason: String,
    },
    
    /// DOC
    NonTerminatingFunction {
        declaration: Span<'src>,
    },
    
    //=============//
    // Suggestions //
    //=============//
    
    /// DOC
    RecommendedFunctionName {
        name: String,
        new_name: String,
        reason: String,
        declaration: Span<'src>,
    }
}

impl<'src> Message<'src> {
    /// DOC
    pub fn level(&self) -> MessageLevel {
        match self {
            Self::UnmatchedOpeningBrace { .. }
                | Self::UnmatchedClosingBrace { .. }
                | Self::ExpectedCompoundFunctionIdentifier { .. }
                | Self::ExpectedCompoundFunctionDefinition { .. }
                | Self::IncompatibleDefinitions { .. }
                | Self::UndefinedFunctionReference { .. }
                => MessageLevel::Error,
            Self::IdentifierContainsKeyword { .. }
                | Self::UnnecessaryBraces { .. }
                | Self::UnusedFunction { .. }
                | Self::EquivalentDefinitions { .. }
                | Self::IneffectiveExpression { .. }
                | Self::UnreachableCode { .. }
                | Self::NonTerminatingFunction { .. }
                => MessageLevel::Warning,
            Self::RecommendedFunctionName { .. }
                => MessageLevel::Suggestion,
        }
    }
    
    /// DOC
    pub fn lines(&self) -> Vec<String> {
        todo!()
    }
    
    /// DOC
    pub fn formatted(&self, indent: usize) -> String {
        todo!()
    }
}



//==========//
// Examples //
//==========//

//=== Errors ===//

// Error:
//   ⬥ unmatched opening brace
//     ┏━❰ path/to/file.un at 1:1 ❱
//     ┃
//   1 ┃ main {
//     ┃      ▔
//     ╹
// Error:
//   ⬥ unmatched closing brace
//     ┏━❰ path/to/file.un at 1:1 ❱
//     ┃
//   1 ┃ func{ - | + }
//     ┃             ▔
//     ╹
// Error:
//   ⬥ expected a function identifier
//   ⬥ found a keyword
//     ┏━❰ path/to/file.un at 1:1 ❱
//     ┃
//   1 ┃ | {
//     ┃ ▔
//     ╹
// Error:
//   ⬥ expected a function identifier
//   ⬥ found an atomic function
//     ┏━❰ path/to/file.un at 1:1 ❱
//     ┃
//   1 ┃ + {
//     ┃ ▔
//     ╹
// Error:
//   ⬥ expected a function identifier
//   ⬥ found a bracketed expression
//     ┏━❰ path/to/file.un at 1:1 ❱
//     ┃
//   1 ┃ { - - +
//     ┃ ▔▔▔▔▔▔▔
//   2 ┃     | - +
//     ┃ ▔▔▔▔▔▔▔▔▔
//   3 ┃     | }
//     ┃ ▔▔▔▔▔▔▔
//     ╹
// Error:
//   ⬥ expected a bracketed definition "{ ... }" for the function "my_func"
//   ⬥ found the token "("
//     ┏━❰ path/to/file.un at 1:1 ❱
//     ┃
//   1 ┃ my_func ( )
//     ┃         ▔
//     ╹
// Error:
//   ⬥ expected a bracketed definition "{ ... }" for the function "my_func"
//   ⬥ found the end of input
//     ┏━❰ path/to/file.un at 1:1 ❱
//     ┃
//   1 ┃ my_func
//     ┃        ▔
//     ╹
// Error:
//   ⬥ incompatible definitions of the function "do_something"
//     ┏━❰ path/to/file.un at 1:1 ❱
//     ┃
//   1 ┃ do_something { - }
//     ┃ ▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔
//     ╹
//     ┏━❰ path/to/another/file.un at 1:1 ❱
//     ┃
//   1 ┃ do_something { - + }
//     ┃ ▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔
//     ╹
// Error:
//   ⬥ reference to an undefined function "main"
//     ┏━❰ path/to/file.un at 1:1 ❱
//     ┃
//   1 ┃ { mann ! }
//     ┃   ▔▔▔▔
//     ╹
//   ⬥ did you mean "main"?



//=== Warnings ===//

// Warning:
//   ⬥ identifiers should not contain the keyword "|"
//     ┏━❰ path/to/file.un at 1:1 ❱
//     ┃
//   1 ┃ main { do|a|thing }
//     ┃        ▔▔▔▔▔▔▔▔▔▔
//     ╹
// Warning:
//   ⬥ identifiers should not contain the keyword "{"
//     ┏━❰ path/to/file.un at 1:1 ❱
//     ┃
//   1 ┃ main { run{ }
//     ┃        ▔▔▔▔
//     ╹
// Warning:
//   ⬥ identifiers should not contain the keyword "}"
//     ┏━❰ path/to/file.un at 1:1 ❱
//     ┃
//   1 ┃ main { run} }
//     ┃        ▔▔▔▔
//     ╹
// Warning:
//   ⬥ unnecessary braces around an empty expression
//     ┏━❰ path/to/file.un at 1:1 ❱
//     ┃
//   1 ┃ main { - { } + }
//     ┃          ▔▔▔
//     ╹
// Warning:
//   ⬥ unnecessary braces around a composition
//     ┏━❰ path/to/file.un at 1:1 ❱
//     ┃
//   1 ┃ main { - { - + } + }
//     ┃          ▔▔▔▔▔▔▔
//     ╹
// Warning:
//   ⬥ unnecessary braces around an alternation
//     ┏━❰ path/to/file.un at 1:1 ❱
//     ┃
//   1 ┃ main { { - - | - + } | + }
//     ┃        ▔▔▔▔▔▔▔▔▔▔▔▔▔
//     ╹
// Warning:
//   ⬥ unused function "func"
//     ┏━❰ path/to/file.un at 1:1 ❱
//     ┃
//   1 ┃ func { + + }
//     ┃ ▔▔▔▔
//     ╹
// Warning:
//   ⬥ multiple equivalent definitions of the function "func"
//     ┏━❰ path/to/file.un at 1:1 ❱
//     ┃
//   1 ┃ func { + + }
//     ┃ ▔▔▔▔▔▔▔▔▔▔▔▔
//     ╹
//     ┏━❰ path/to/another/file.un at 1:1 ❱
//     ┃
//   2 ┃ func { + { + - + } }
//     ┃ ▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔
//     ╹
// Warning:
//   ⬥ ineffective expression
//     ┏━❰ path/to/file.un at 1:1 ❱
//     ┃
//   1 ┃ main { + - }
//     ┃        ▔▔▔
//     ╹
// Warning:
//   ⬥ ineffective expression
//     ┏━❰ path/to/file.un at 1:1 ❱
//     ┃
//   1 ┃ main { + *4 - - /2 - /2 }
//     ┃        ▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔
//     ╹
// Warning:
//   ⬥ ineffective expression
//     ┏━❰ path/to/file.un at 1:1 ❱
//     ┃
//   1 ┃ main { try-thing | - + | }
//     ┃                    ▔▔▔▔▔
//     ╹
// Warning:
//   ⬥ ineffective expression
//     ┏━❰ path/to/file.un at 1:1 ❱
//     ┃
//   1 ┃ main { fail-if-zero | fail-if-zero | ... }
//     ┃                     ▔▔▔▔▔▔▔▔▔▔▔▔▔▔
//     ╹
// Warning:
//   ⬥ unreachable code following a never-failing expression
//     ┏━❰ path/to/file.un at 1:1 ❱
//     ┃
//   1 ┃ main { - + | | + + }
//     ┃              ▔▔▔▔▔
//     ╹
// Warning:
//   ⬥ unreachable code following a never-failing expression
//     ┏━❰ path/to/file.un at 1:1 ❱
//     ┃
//   1 ┃ main { - + fail-if-zero | fail-if-nonzero | + + }
//     ┃                                           ▔▔▔▔▔
//     ╹
// Warning:
//   ⬥ unreachable code following an always-failing expression
//     ┏━❰ path/to/file.un at 1:1 ❱
//     ┃
//   1 ┃ main { 0 - - + | }
//     ┃            ▔▔▔
//     ╹
// Warning:
//   ⬥ unreachable code following an always-failing expression
//     ┏━❰ path/to/file.un at 1:1 ❱
//     ┃
//   1 ┃ main { fail-if-zero | fail-if-zero + + | }
//     ┃                                    ▔▔▔
//     ╹
// Warning:
//   ⬥ unreachable code following a non-terminating expression
//     ┏━❰ path/to/file.un at 1:1 ❱
//     ┃
//   1 ┃ main { loop - + | }
//     ┃             ▔▔▔▔▔
//     ╹
// Warning:
//   ⬥ unreachable code following a non-terminating expression
//     ┏━❰ path/to/file.un at 1:1 ❱
//     ┃
//   1 ┃ main { - + loop-if-nonzero | loop-if-zero - + | }
//     ┃                                           ▔▔▔▔▔
//     ╹
// Warning:
//   ⬥ non-effectful non-terminating function "main" on input 0
//     ┏━❰ path/to/file.un at 1:1 ❱
//     ┃
//   1 ┃ main { fail-if-nonzero main }
//     ┃ ▔▔▔▔
//     ╹
