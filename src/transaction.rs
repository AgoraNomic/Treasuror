#![allow(unused_assignments)]

use chrono::naive::{NaiveDate, NaiveDateTime};

use crate::token::{Token, TokenIterator, Operator};

macro_rules! match_first_pop {
    ($v:ident { $( $t:pat => $b:block ),+, } else $e:block) => {{
        let tmp_first = $v[0];
        match tmp_first {
            $(
                $t => {
                    $v.remove(0);
                    $b
                },)*
            _ => $e,
        }
    }}
}

pub struct Transaction<'a> {
    datetime: NaiveDateTime,
    agent: String,
    amount: Amount,
    action: Operator<'a>,
    comment: String,
}

impl<'a> Transaction<'a> {
    pub fn with_date_from_str(date: &NaiveDate, mut ln: String) -> Option<Transaction<'a>> {
        if ln.is_empty() {
            return None;
        }
        ln.push('\n');
        let mut tokens: Vec<Token> = TokenIterator::from_str(&ln).collect();

        Some(Transaction {
            datetime: match_first_pop!(tokens {
                Token::Time(t) => { date.and_time(t) },
            } else { date.and_hms(0,0,0) }),
            agent: match_first_pop!(tokens {
                Token::Identifier(i) => { String::from(i) },
            } else { return None }),
            amount: Amount::from_vec(&mut tokens),
            action: match_first_pop!(tokens {
                Token::Op(o) => { o },
            } else { return None }),
            comment: match_first_pop!(tokens {
                Token::String(s) => { String::from(s) },
            } else { String::from("") }),
        })
    }
   

    pub fn get_datetime(&self) -> &NaiveDateTime {
        &self.datetime
    }

    pub fn get_agent(&self) -> &str {
        &self.agent
    }

    pub fn get_amount(&self) -> Amount {
        self.amount
    }

    pub fn get_action(&self) -> &Operator {
        &self.action
    }

    pub fn get_comment(&self) -> &str {
        &self.comment
    }
}

#[derive(Copy, Clone)]
pub enum Amount {
    Everything,
    AllOf(Currency),
    PartOf(Currency, u32),
}

impl Amount {
    pub fn from_vec(s: &mut Vec<Token>) -> Amount {
        match s[0] {
            Token::Integer(i) => {
                s.remove(0);
                Amount::PartOf(
                    Currency::from_str(s.remove(1).extract_string()).expect("invalid currency type"),
                    i)
            },
            Token::Blob => match s[1] {
                Token::Identifier(s) => Amount::AllOf(Currency::from_str(&s).expect("invalid currency type")),
                _ => Amount::Everything,
            },
            _ => panic!("invalid token"),
        }
    }

    pub fn pretty(&self) -> &str {
        match self {
            Amount::Everything => "everything",
            Amount::AllOf(_) => "all of one",
            Amount::PartOf(_, _) => "part of one",
        }
    }
}

#[derive(Copy, Clone)]
pub enum Currency {
    Coin,
    WinCard,
    JusticeCard,
    LegiCard,
    VoteCard,
    WinPoint,
    BlotBGone,
    Pendant,
    ExtraVote,
}

impl Currency {
    pub fn from_str(s: &str) -> Option<Currency> {
        match s {
            "cn" => Some(Currency::Coin),
            "wc" => Some(Currency::WinCard),
            "jc" => Some(Currency::JusticeCard),
            "lc" => Some(Currency::LegiCard),
            "vc" => Some(Currency::VoteCard),
            "wp" => Some(Currency::WinPoint),
            "bg" => Some(Currency::BlotBGone),
            "pd" => Some(Currency::Pendant),
            "xv" => Some(Currency::ExtraVote),
            _ => None,
        }
    }
}

/* struct AgoranEntity<'a> {
   name: &'a str,
   balances: HashMap<&'a Currency, u32>,
   } */
