use super::unit::{Currency, FullUnit};
use super::token::Token;

#[derive(Copy, Clone)]
pub enum Amount {
    Everything,
    AllOf(Currency),
    PartOf(FullUnit, u32),
}

impl Amount {
    pub fn from_vec(s: &mut Vec<Token>) -> Amount {
        match s[0] {
            Token::Integer(i) => {
                s.remove(0);
                Amount::PartOf(
                    FullUnit::from_vec(s), // s.remove(0).extract_string()).unwrap(),
                    i)
            },
            Token::Blob => match s[1] {
                Token::Identifier(i) => {
                    s.remove(1);
                    s.remove(0);
                    Amount::AllOf(Currency::from_str(&i).unwrap())
                },
                _ => {
                    s.remove(0);
                    Amount::Everything
                },
            },
            _ => panic!("invalid token"),
        }
    }

    pub fn pretty(&self) -> String {
        match self {
            Amount::Everything => String::from("everything"),
            Amount::AllOf(c) => String::from("all of ") + c.abbr(),
            Amount::PartOf(c, a) => a.to_string() + match c {
                FullUnit::Bare(_) => "",
                FullUnit::Boatload(_) => "bl:"
            } + c.currency().abbr(),
        }
    }
}
