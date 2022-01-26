use crate::parser::{
    common::Token,
    error::syntax::SyntaxResult
};

pub trait Parseable {
    fn from_tokens(tokens: &mut Vec<Token>) -> SyntaxResult<Self> where Self: Sized;
}
