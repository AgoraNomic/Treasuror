use chrono::{
    format::ParseError as ChronoParseError,
    naive::NaiveTime,
};

use nom::{
    IResult,
    bytes::complete::{take_while, take_till},
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
pub enum ParseError<I> {
    Chrono(ChronoParseError),
    Nom(NomError<I>),
}

pub fn is_id_char(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '.' || c == '_' || c == '&'
}

impl<I> ParseErrorTrait<I> for ParseError<I> {
    fn from_error_kind(input: I, kind: ErrorKind) -> Self {
        ParseError::Nom(NomError::new(input, kind))
    }

    fn append(_: I, _: ErrorKind, other: Self) -> Self {
        other
    }
}

pub fn bracketed(s: &str) -> IResult<&str, &str> {
    delimited(char('['), take_till(|c| c == ']'), char(']'))(s)
}

pub fn token_time(s: &str) -> IResult<&str, NaiveTime, ParseError<&str>> {
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

pub fn token_identifier(s: &str) -> IResult<&str, &str> {
    take_while(is_id_char)(s)
}

#[cfg(test)]
mod tests {
    use chrono::naive::NaiveTime;
    use super::*;

    #[test]
    fn bracketed_test() {
        assert_eq!(bracketed("[arst]dhneio"), Ok(("dhneio", "arst")))
    }

    #[test]
    fn bracketed_test_fail() {
        assert!(bracketed("(arst)dhneio").is_err())
    }

    #[test]
    fn time_test() {
        assert_eq!(token_time("[12:34]"), Ok(("", NaiveTime::from_hms(12, 34, 0))))
    }

    #[test]
    fn time_test_invalid_time() {
        assert!(token_time("[32:33]").is_err())
    }

    #[test]
    fn time_test_bad_format() {
        assert!(token_time("[12;34").is_err())
    }

    #[test]
    fn identifier_test() {
        assert_eq!(token_identifier("Trigon 5cn>Aris"), Ok((" 5cn>Aris", "Trigon")))
    }

    #[test]
    fn identifier_test_weird_chars() {
        assert_eq!(token_identifier("L&F_Dept. 331cn+"), Ok((" 331cn+", "L&F_Dept.")))
    }

    #[test]
    fn identifier_test_numbers() {
        assert_eq!(token_identifier("Trigon12"), Ok(("12", "Trigon")))
    }
}
