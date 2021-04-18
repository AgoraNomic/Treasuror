use chrono::{
    format::ParseError as ChronoParseError,
    naive::NaiveTime,
    token::Token,
};

use nom::{
    IResult,
    bytes::complete::take_till,
    character::complete::char,
    Err as NomErr,
    error::{
        Error as NomError,
        ErrorKind,
        ParseError as ParseErrorTrait,
    },
    sequence::delimited,
};

#[derive(Debug, PartialEq)]
enum ParseError<I> {
    Chrono(ChronoParseError),
    Nom(NomError<I>),
}

impl<I> ParseErrorTrait<I> for ParseError<I> {
    fn from_error_kind(input: I, kind: ErrorKind) -> Self {
        ParseError::Nom(NomError::new(input, kind))
    }

    fn append(_: I, _: ErrorKind, other: Self) -> Self {
        other
    }
}

fn bracketed(s: &str) -> IResult<&str, &str> {
    delimited(char('['), take_till(|c| c == ']'), char(']'))(s)
}

fn take_time(s: &str) -> IResult<&str, NaiveTime, ParseError<&str>> {
    match bracketed(s) {
        Ok((after, time_str)) => {
            match NaiveTime::parse_from_str(time_str, "%R") {
                Ok(time) => Ok((after, time)),
                Err(cpe) => Err(NomErr::Error(ParseError::Chrono(cpe))),
            }
        }
        Err(e) => Err(e.map(ParseError::Nom)),
    }
}
