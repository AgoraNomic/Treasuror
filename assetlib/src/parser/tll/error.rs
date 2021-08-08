#[derive(Clone, Debug)]
pub struct SyntaxError {
    pub message: String,
    pub kind: ErrorKind,
}

impl SyntaxError {
    pub fn empty(kind: ErrorKind) -> SyntaxError {
        SyntaxError { message: String::new(), kind }
    }

    pub fn from(message: &str, kind: ErrorKind) -> SyntaxError {
        SyntaxError { message: message.to_string(), kind }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ErrorKind {
    Empty,
    ExpectedIdentifier,
    ExpectedInteger,
    ExpectedOperator,
    ExpectedStringlike,
    UnrecognizedCommand,
}
