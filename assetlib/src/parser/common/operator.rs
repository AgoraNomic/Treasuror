use crate::parser::tll::Trade;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Operator {
    Plus,
    Minus,
    Transfer(String),
    Trade(Trade),
}
