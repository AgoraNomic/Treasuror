#[derive(Copy, Clone)]
pub enum Operator<'a> {
    Plus,
    Minus,
    Transfer(&'a str),
}
