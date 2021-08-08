#[derive(Clone, Debug)]
pub struct SyntaxError {
    pub message: String,
    pub kind: ErrorKind,
}

#[derive(Clone, Copy, Debug)]
pub enum ErrorKind {
    Empty,
}
