use std::num::{ParseFloatError, ParseIntError};

use chrono::{format::ParseError as ChronoParseError, naive::NaiveTime};

use nom::{
    bytes::complete::{take_till, take_while},
    character::complete::char,
    combinator::recognize,
    error::{Error as NomError, ErrorKind, ParseError as ParseErrorTrait},
    sequence::delimited,
    Err as NomErr, IResult,
};

use super::Token;

#[derive(Debug, PartialEq)]
pub enum ParseError<I> {
    Chrono(ChronoParseError),
    Nom(NomError<I>),
    Int(ParseIntError),
    Float(ParseFloatError),
}

pub type TokenIResult<'a> = IResult<&'a str, Token, ParseError<&'a str>>;
pub type StringIResult<'a> = IResult<&'a str, &'a str, ParseError<&'a str>>;

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

impl<I> From<ParseIntError> for ParseError<I> {
    fn from(e: ParseIntError) -> ParseError<I> {
        ParseError::Int(e)
    }
}

impl<I> From<ParseFloatError> for ParseError<I> {
    fn from(e: ParseFloatError) -> ParseError<I> {
        ParseError::Float(e)
    }
}

// impl<I, T: ParseErrorTrait<I>> From<T> for ParseError<I> {
//    fn from(error: T) {
//        ParseError::Nom(error)
//    }
//}

pub fn bracketed(s: &str) -> StringIResult {
    delimited(char('['), take_till(|c| c == ']'), char(']'))(s)
}

pub fn token_time(s: &str) -> TokenIResult {
    match bracketed(s) {
        Ok((after, time_str)) => NaiveTime::parse_from_str(time_str, "%R")
            .map(|time| (after, Token::Time(time)))
            .map_err(|cpe| NomErr::Error(ParseError::Chrono(cpe))),
        Err(e) => Err(e.into()),
    }
}

pub fn token_identifier(s: &str) -> TokenIResult {
    take_while(is_id_char)(s).map(|(rest, id)| (rest, Token::Identifier(id.to_string())))
}

pub fn token_integer(s: &str) -> TokenIResult {
    match take_while(|c: char| c.is_digit(10))(s) {
        Ok((rest, digits)) => match digits.parse::<u32>() {
            Ok(i) => Ok((rest, i.into())),
            Err(pie) => Err(NomErr::Error(pie.into())),
        }
        Err(e) => Err(e.into()),
    }
}

pub fn token_blob(s: &str) -> TokenIResult {
    char('*')(s).map(|(rest, _)| (rest, Token::Blob))
}

pub fn token_separator(s: &str) -> TokenIResult {
    char(':')(s).map(|(rest, _)| (rest, Token::Separator))
}

pub fn token_float(s: &str) -> TokenIResult {
    match recognize(delimited(
        take_while(|c: char| c.is_digit(10)),
        char('.'),
        take_while(|c: char| c.is_digit(10)),
    ))(s)
    {
        Ok((rest, digits)) => {
            match digits.parse::<f32>() {
                Ok(f) => Ok((rest, f.into())),
                Err(e) => Err(NomErr::Error(e.into())),
            }
        }
        Err(e) => Err(e.into()),
    }
}

pub fn token_string(s: &str) -> IResult<&str, Token> {
    delimited(char('"'), take_till(|c| c == '"'), char('"'))(s)
        .map(|(rest, s)| (rest, Token::String(s.to_string())))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::naive::NaiveTime;

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
        assert_eq!(
            token_time("[12:34]"),
            Ok(("", Token::Time(NaiveTime::from_hms(12, 34, 0))))
        )
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
        assert_eq!(
            token_identifier("Trigon 5cn>Aris"),
            Ok((" 5cn>Aris", Token::Identifier("Trigon".to_string())))
        )
    }

    #[test]
    fn identifier_test_weird_chars() {
        assert_eq!(
            token_identifier("L&F_Dept. 331cn+"),
            Ok((" 331cn+", Token::Identifier("L&F_Dept.".to_string())))
        )
    }

    #[test]
    fn identifier_test_numbers() {
        assert_eq!(
            token_identifier("Trigon12"),
            Ok(("12", Token::Identifier("Trigon".to_string())))
        )
    }

    #[test]
    fn integer_test() {
        assert_eq!(token_integer("112"), Ok(("", Token::Integer(112))))
    }

    #[test]
    fn integer_test_float() {
        assert_eq!(token_integer("1.205"), Ok((".205", Token::Integer(1))))
    }

    #[test]
    fn integer_test_currency() {
        assert_eq!(token_integer("15bl:cn"), Ok(("bl:cn", Token::Integer(15))))
    }

    #[test]
    fn blob_test() {
        assert_eq!(token_blob("*cn"), Ok(("cn", Token::Blob)))
    }

    #[test]
    fn separator_test() {
        assert_eq!(token_separator(":cn"), Ok(("cn", Token::Separator)))
    }

    #[test]
    fn float_test() {
        assert_eq!(token_float("123.321"), Ok(("", Token::Float(123.321))))
    }

    #[test]
    fn string_test() {
        assert_eq!(
            token_string(r#""boatload""#),
            Ok(("", Token::String("boatload".to_string())))
        )
    }
}
