pub type SyntaxResult<T> = Result<T, SyntaxError>;

#[derive(Clone, Debug)]
pub struct SyntaxError {
    pub message: String,
    pub kind: ErrorKind,
}

impl SyntaxError {
    pub fn empty(kind: ErrorKind) -> SyntaxError {
        SyntaxError {
            message: String::new(),
            kind,
        }
    }

    pub fn from(message: &str, kind: ErrorKind) -> SyntaxError {
        SyntaxError {
            message: message.to_string(),
            kind,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ErrorKind {
    Empty,
    ExpectedBlob,
    ExpectedCommand,
    ExpectedFloat,
    ExpectedIdentifier,
    ExpectedInteger,
    ExpectedOperator,
    ExpectedSeparator,
    ExpectedStringlike,
    ExpectedTime,
    IncompleteAmount,
    IncompleteUnit,
    InvalidCurrency,
    InvalidEntityType,
    InvalidPrefix,
    UnrecognizedCommand,
    UnrecognizedDirective,
}
