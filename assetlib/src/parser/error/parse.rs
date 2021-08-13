use std::num::{ParseFloatError, ParseIntError};

use chrono::format::ParseError as ChronoParseError;

use nom::{
    error::{Error as NomError, ErrorKind, ParseError as ParseErrorTrait},
    Err as NomErr, IResult,
};

use crate::parser::common::Token;

pub type TokenIResult<'a> = IResult<&'a str, Token, ParseError<&'a str>>;
pub type StringIResult<'a> = IResult<&'a str, &'a str, ParseError<&'a str>>;

pub type ParseResult<'a, T> = Result<T, ParseError<&'a str>>;

#[derive(Debug, PartialEq)]
pub enum ParseError<I> {
    Chrono(ChronoParseError),
    Nom(NomError<I>),
    Int(ParseIntError),
    Float(ParseFloatError),
    Unparseable(String),
}

impl<I> ParseErrorTrait<I> for ParseError<I> {
    fn from_error_kind(input: I, kind: ErrorKind) -> Self {
        ParseError::Nom(NomError::new(input, kind))
    }

    fn append(_: I, _: ErrorKind, other: Self) -> Self {
        other
    }
}

impl<I> From<ParseIntError> for ParseError<I> {
    fn from(e: ParseIntError) -> ParseError<I> {
        ParseError::Int(e)
    }
}

impl<I> From<ChronoParseError> for ParseError<I> {
    fn from(e: ChronoParseError) -> ParseError<I> {
        ParseError::Chrono(e)
    }
}

impl<I> From<ParseFloatError> for ParseError<I> {
    fn from(e: ParseFloatError) -> ParseError<I> {
        ParseError::Float(e)
    }
}

pub fn to_nom_err<I, E: Into<ParseError<I>>>(e: E) -> NomErr<ParseError<I>> {
    NomErr::Error(e.into())
}
