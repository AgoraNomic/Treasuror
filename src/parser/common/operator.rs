#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Operator {
    Plus,
    Minus,
    Transfer(String),
}
