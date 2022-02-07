use super::unit::{Currency, FullUnit};
use crate::parser::common::parseable_prelude::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Amount {
    Everything,
    AllOf(Currency),
    PartOf(FullUnit, u32),
}

impl Amount {
    pub fn pretty(&self) -> String {
        match self {
            Amount::Everything => String::from("everything"),
            Amount::AllOf(c) => String::from("all of ") + c.abbr(),
            Amount::PartOf(c, a) => {
                a.to_string()
                    + match c {
                        FullUnit::Bare(_) => "",
                        FullUnit::Boatload(_) => "bl:",
                    }
                    + c.currency().abbr()
            }
        }
    }
}

impl Parseable for Amount {
    fn from_tokens(tokens: &mut Vec<Token>) -> SyntaxResult<Amount> {
        if let Ok(i) = take_integer(tokens, "") {
            Ok(Amount::PartOf(take_full_unit(tokens)?, i))
        } else if let Ok(()) = take_blob(tokens, "") {
            if let Ok(c) = take_identifier(tokens, "") {
                Ok(Amount::AllOf(try_into_currency(&c)?))
            } else {
                Ok(Amount::Everything)
            }
        } else {
            Err(SyntaxError::from(
                "expected integer or blob at start of amount",
                ErrorKind::IncompleteAmount,
            ))
        }
    }
}
