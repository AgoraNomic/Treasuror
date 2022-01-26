pub mod combinators;
mod operator;
mod token;
mod parseable;

pub use crate::parser::common::{
    operator::Operator,
    parseable::Parseable,
    token::{combinators as token_com, Token, TokenIterator},
};

pub mod parseable_prelude {
    pub use crate::parser::{
        common::{token_com::*, Token, Parseable},
        error::syntax::{ErrorKind, SyntaxError, SyntaxResult},
    };
}
