use chrono::naive::NaiveTime;

use nom::{
    Err as NomErr,
    error::Error as NomError,
};

use super::{combinators as com, operator::Operator, error::ParseError};

pub struct TokenIterator<'a> {
    source: &'a str,
}

impl<'a> TokenIterator<'a> {
    pub fn with_source(s: &'a str) -> TokenIterator<'a> {
        TokenIterator { source: s }
    }
}

impl<'a> Iterator for TokenIterator<'a> {
    type Item = Result<Token, ParseError<&'a str>>;

    fn next(&mut self) -> Option<Self::Item> {
        let rest = self.source.trim();

        match com::token_any(rest) {
            Ok((rest2, matched)) => {
                self.source = rest2;
                Some(Ok(matched))
            }
            Err(e) => {
                match e {
                NomErr::Error(ParseError::Nom(n)) => {
                    match n {
                        NomError { input: "", .. } => None,
                        NomError { input: i, .. } => Some(Err(ParseError::Unparseable(i.to_string()))),
                    }
                }
                NomErr::Error(e) | NomErr::Failure(e) => Some(Err(e)),
                _ => None,
            }}
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Time(NaiveTime),
    Identifier(String),
    Integer(u32),
    Blob,
    Separator,
    Float(f32),
    String(String),
    Op(Operator),
    Command(String),
}

impl Token {
    pub fn extract_float(&self) -> f32 {
        if let Token::Float(f) = self {
            *f
        } else {
            panic!("cannot extract float");
        }
    }

    pub fn extract_int(&self) -> u32 {
        if let Token::Integer(i) = self {
            *i
        } else {
            panic!("cannot extract int");
        }
    }

    pub fn extract_string(&self) -> &str {
        match self {
            Token::Identifier(s) | Token::String(s) | Token::Command(s) => s,
            _ => panic!("cannot extract string"),
        }
    }

    pub fn extract_operator(&self) -> &Operator {
        if let Token::Op(ref o) = &self {
            o
        } else {
            panic!("cannot extract operator");
        }
    }
}

impl From<NaiveTime> for Token {
    fn from(dt: NaiveTime) -> Token {
        Token::Time(dt)
    }
}

impl From<u32> for Token {
    fn from(i: u32) -> Token {
        Token::Integer(i)
    }
}

impl From<f32> for Token {
    fn from(f: f32) -> Token {
        Token::Float(f)
    }
}

impl From<Operator> for Token {
    fn from(o: Operator) -> Token {
        Token::Op(o)
    }
}

pub mod combinators {
    use chrono::naive::NaiveTime;

    use crate::match_first_pop;

    use super::{Operator, Token};

    use crate::parser::tll::error::*;
    use crate::model::{Amount, Currency, FullUnit};

    pub fn expect_amount<'a>(tokens: &'a mut Vec<Token>) -> Result<Amount, SyntaxError> {
        if let Ok(i) = expect_integer(tokens, "") {
            Ok(Amount::PartOf(expect_full_unit(tokens)?, i))
        } else if let Ok(()) = expect_blob(tokens, "") {
            if let Ok(c) = expect_identifier(tokens, "") {
                Ok(Amount::AllOf(try_into_currency(&c)?))
            } else {
                Ok(Amount::Everything)
            }
        } else {
            Err(SyntaxError::from("expected integer or blob at start of amount", ErrorKind::IncompleteAmount))
        }
    }

    pub fn expect_blob<'a>(tokens: &'a mut Vec<Token>, message: &'a str) -> Result<(), SyntaxError> {
        match_first_pop!(tokens {
            Token::Blob => { Ok(()) },
        } else {
            Err(SyntaxError::from(
                message,
                ErrorKind::ExpectedBlob
            ))
        })
    }

    pub fn expect_command<'a>(tokens: &'a mut Vec<Token>, message: &'a str) -> Result<String, SyntaxError> {
        match_first_pop!(tokens {
            Token::Command(s) => { Ok(s) },
        } else {
            return Err(SyntaxError::from(
                message,
                ErrorKind::ExpectedCommand
            ));
        })
    }

    pub fn expect_full_unit<'a>(tokens: &'a mut Vec<Token>) -> Result<FullUnit, SyntaxError> {
        if let Ok(i1) = expect_identifier(tokens, "") {
            if let Ok(()) = expect_separator(tokens, "") {
                if let Ok(i2) = expect_identifier(tokens, "") {
                    if i1 == "bl" {
                        Ok(FullUnit::Boatload(try_into_currency(&i2)?))
                    } else {
                        Err(SyntaxError::from(&format!("invalid prefix {:?} in unit", i1), ErrorKind::InvalidPrefix))
                    }
                } else {
                    Err(SyntaxError::from("expected currency after separator in unit", ErrorKind::IncompleteUnit))
                }
            } else {
                Ok(FullUnit::Bare(try_into_currency(&i1)?))
            }
        } else {
            Err(SyntaxError::from("expected identifier to begin unit", ErrorKind::IncompleteUnit))
        }
    }

    pub fn expect_identifier<'a>(tokens: &'a mut Vec<Token>, message: &'a str) -> Result<String, SyntaxError> {
        match_first_pop!(tokens {
            Token::Identifier(s) => { Ok(s) },
        } else {
            return Err(SyntaxError::from(
                message,
                ErrorKind::ExpectedIdentifier
            ));
        })
    }

    pub fn expect_integer<'a>(tokens: &'a mut Vec<Token>, message: &'a str) -> Result<u32, SyntaxError> {
        match_first_pop!(tokens {
            Token::Integer(i) => { Ok(i) },
        } else {
            return Err(SyntaxError::from(
                message,
                ErrorKind::ExpectedIdentifier
            ));
        })
    }

    pub fn expect_separator<'a>(tokens: &'a mut Vec<Token>, message: &'a str) -> Result<(), SyntaxError> {
        match_first_pop!(tokens {
            Token::Separator => { Ok(()) },
        } else {
            Err(SyntaxError::from(
                message,
                ErrorKind::ExpectedSeparator
            ))
        })
    }

    pub fn expect_operator<'a>(tokens: &'a mut Vec<Token>, message: &'a str) -> Result<Operator, SyntaxError> {
        match_first_pop!(tokens {
            Token::Op(o) => { Ok(o) },
        } else {
            return Err(SyntaxError::from(
                message,
                ErrorKind::ExpectedOperator
            ));
        })
    }

    pub fn expect_stringlike<'a>(tokens: &'a mut Vec<Token>, message: &'a str) -> Result<String, SyntaxError> {
        match_first_pop!(tokens {
            Token::String(s) => { Ok(s) },
            Token::Identifier(s) => { Ok(s) },
        } else {
            return Err(SyntaxError::from(
                message,
                ErrorKind::ExpectedStringlike
            ));
        })
    }

    pub fn expect_time<'a>(tokens: &'a mut Vec<Token>, message: &'a str) -> Result<NaiveTime, SyntaxError> {
        match_first_pop!(tokens {
            Token::Time(t) => { Ok(t) },
        } else {
            return Err(SyntaxError::from(
                message,
                ErrorKind::ExpectedTime
            ));
        })
    }

    pub fn try_into_currency(s: &str) -> Result<Currency, SyntaxError> {
        Currency::from_abbr(s).ok_or_else(|| SyntaxError::from(&format!("unrecognized currency abbreviation: {}", s), ErrorKind::InvalidCurrency))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::naive::NaiveTime;

    #[test]
    fn test_tll_1() {
        assert_eq!(
            TokenIterator::with_source("[01:23] Trigon 5bl:cn+").collect::<Vec<Token>>(),
            vec![
                Token::Time(NaiveTime::from_hms(1, 23, 0)),
                Token::Identifier("Trigon".to_string()),
                Token::Integer(5),
                Token::Identifier("bl".to_string()),
                Token::Separator,
                Token::Identifier("cn".to_string()),
                Token::Op(Operator::Plus),
            ]
        )
    }

    #[test]
    fn test_tll_2() {
        assert_eq!(
            TokenIterator::with_source("[21:52] Cuddlebeam *>CB_Locker").collect::<Vec<Token>>(),
            vec![
                Token::Time(NaiveTime::from_hms(21, 52, 0)),
                Token::Identifier("Cuddlebeam".to_string()),
                Token::Blob,
                Token::Op(Operator::Transfer("CB_Locker".to_string())),
            ]
        )
    }

    #[test]
    fn test_gdsl_1() {
        assert_eq!(
            TokenIterator::with_source("FLOTATION 10.0000").collect::<Vec<Token>>(),
            vec![
                Token::Identifier("FLOTATION".to_string()),
                Token::Float(10.0f32),
            ]
        )
    }

    #[test]
    fn test_gdsl_2() {
        assert_eq!(
            TokenIterator::with_source(r#"ENT P L&F_Dept. "Lost and Found Department" 12024cn"#)
                .collect::<Vec<Token>>(),
            vec![
                Token::Identifier("ENT".to_string()),
                Token::Identifier("P".to_string()),
                Token::Identifier("L&F_Dept.".to_string()),
                Token::String("Lost and Found Department".to_string()),
                Token::Integer(12024),
                Token::Identifier("cn".to_string()),
            ]
        )
    }
}
