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

        println!("{}", rest);
        match com::token_any(rest) {
            Ok((rest2, matched)) => {
                self.source = rest2;
                Some(Ok(matched))
            }
            Err(e) => {
                println!("{:?}", e);
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
