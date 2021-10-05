use chrono::naive::{NaiveTime, NaiveDate};

use nom::{
    branch::alt,
    bytes::complete::{take_till, take_while, take_while1},
    character::complete::char,
    combinator::recognize,
    sequence::{delimited, preceded},
};

use super::{Operator, Token};
use crate::parser::error::parse::{to_nom_err, StringIResult, TokenIResult};

pub fn is_id_char(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '.' || c == '_' || c == '&'
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
    let (after, time_str) = bracketed(s)?;
    let time = NaiveTime::parse_from_str(time_str, "%R").map_err(to_nom_err)?;
    Ok((after, time.into()))
}

pub fn token_date(s: &str) -> TokenIResult {
    let (after, date_str) = bracketed(s)?;
    let date = NaiveDate::parse_from_str(date_str, "%F").map_err(to_nom_err)?;
    Ok((after, date.into()))
}

pub fn identifier(s: &str) -> StringIResult {
    take_while1(is_id_char)(s)
}

pub fn token_identifier(s: &str) -> TokenIResult {
    identifier(s).map(|(rest, matched)| (rest, Token::Identifier(matched.to_string())))
}

pub fn token_integer(s: &str) -> TokenIResult {
    let (rest, digits) = take_while(|c: char| c.is_digit(10))(s)?;
    let i = digits.parse::<u32>().map_err(to_nom_err)?;
    Ok((rest, i.into()))
}

pub fn token_blob(s: &str) -> TokenIResult {
    char('*')(s).map(|(rest, _)| (rest, Token::Blob))
}

pub fn token_separator(s: &str) -> TokenIResult {
    char(':')(s).map(|(rest, _)| (rest, Token::Separator))
}

pub fn token_float(s: &str) -> TokenIResult {
    let (rest, digits) = recognize(delimited(
        take_while(|c: char| c.is_digit(10)),
        char('.'),
        take_while(|c: char| c.is_digit(10)),
    ))(s)?;

    let f = digits.parse::<f32>().map_err(to_nom_err)?;
    Ok((rest, f.into()))
}

pub fn token_string(s: &str) -> TokenIResult {
    delimited(char('"'), take_till(|c| c == '"'), char('"'))(s)
        .map(|(rest, s)| (rest, Token::String(s.to_string())))
}

pub fn token_plus(s: &str) -> TokenIResult {
    char('+')(s).map(|(rest, _)| (rest, Operator::Plus.into()))
}

pub fn token_minus(s: &str) -> TokenIResult {
    char('-')(s).map(|(rest, _)| (rest, Operator::Minus.into()))
}

pub fn token_transfer(s: &str) -> TokenIResult {
    preceded(char('>'), identifier)(s)
        .map(|(rest, t)| (rest, Token::Op(Operator::Transfer(t.to_string()))))
}

pub fn token_operator(s: &str) -> TokenIResult {
    alt((token_plus, alt((token_minus, token_transfer))))(s)
}

pub fn token_command(s: &str) -> TokenIResult {
    preceded(char('#'), identifier)(s).map(|(rest, t)| (rest, Token::Command(t.to_string())))
}

pub fn token_any(s: &str) -> TokenIResult {
    alt((
        token_time,
        alt((
            token_date,
            alt((
            token_identifier,
            alt((
                token_float,
                alt((
                    token_integer,
                    alt((
                        token_string,
                        alt((
                            token_operator,
                            alt((token_blob, alt((token_separator, token_command)))),
                        )),
                    )),
                )),
            )),
        )),
        )),
    ))(s)
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
    fn identifier_str_test() {
        assert_eq!(identifier("Trigon 5cn>Aris"), Ok((" 5cn>Aris", "Trigon")))
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

    #[test]
    fn plus_test() {
        assert_eq!(
            token_operator("+5cn"),
            Ok(("5cn", Token::Op(Operator::Plus)))
        )
    }

    #[test]
    fn minus_test() {
        assert_eq!(
            token_operator("-10cn"),
            Ok(("10cn", Token::Op(Operator::Minus)))
        )
    }

    #[test]
    fn transfer_test() {
        assert_eq!(
            token_operator(">Cuddlebeam"),
            Ok(("", Token::Op(Operator::Transfer("Cuddlebeam".to_string()))))
        )
    }

    #[test]
    fn command_test() {
        assert_eq!(
            token_command("#report"),
            Ok(("", Token::Command("report".to_string()))),
        )
    }
}
