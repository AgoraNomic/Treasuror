use super::{
    common::error::*,
    tll::error::*,
};

#[derive(Debug)]
pub enum AnyError<I> {
    Parse(ParseError<I>),
    Syntax(SyntaxError),
}

impl<I> From<SyntaxError> for AnyError<I> {
    fn from(e: SyntaxError) -> Self {
        AnyError::Syntax(e)
    }
}

impl<I> From<ParseError<I>> for AnyError<I> {
    fn from(e: ParseError<I>) -> Self {
        AnyError::Parse(e)
    }
}
