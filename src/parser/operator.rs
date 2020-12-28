#[derive(Clone)]
pub enum Operator {
    Plus,
    Minus,
    Transfer(String),
}
