#![allow(unused_assignments)]

use chrono::naive::{NaiveDate, NaiveDateTime};

use crate::token::{Token, TokenIterator, Operator, Currency};

macro_rules! match_increment {
    ($v:ident in $i:ident { $( $t:pat => $b:block ),+, } else $e:block) => { match $v {
        $(
            Some($t) => {
                $v = $i.next();
                $b
            },)*
        Some(_) => $e,
        None => $e,
    }}
}

pub struct Transaction/*<'a>*/ {
    datetime: NaiveDateTime,
    agent: String,
    amount: u32,
    currency: Currency,
    action: Operator,
    comment: String,
//    agent: &'a AgoranEntity,
}

impl Transaction {
    pub fn with_date_from_str(date: &NaiveDate, mut ln: String) -> Option<Transaction> {
        if ln.is_empty() {
            return None;
        }
        ln.push('\n');
        let mut tokens = TokenIterator::from_str(&ln);
        let mut current_token = tokens.next();

        Some(Transaction {
            datetime: match_increment!(current_token in tokens {
                Token::Time(t) => { date.and_time(t) },
            } else { date.and_hms(0,0,0) }),
            agent: match_increment!(current_token in tokens {
                Token::Identifier(i) => { i },
            } else { return None }),
            amount: match_increment!(current_token in tokens {
                Token::Integer(i) => { i },
                Token::Blob => { 10000 },
            } else { return None }),
            currency: match_increment!(current_token in tokens {
                Token::Curr(c) => { c },
            } else { Currency::Coin }),
            action: match_increment!(current_token in tokens {
                Token::Op(o) => { o },
            } else { return None }),
            comment: match_increment!(current_token in tokens {
                Token::String(s) => { s },
            } else { String::from("") }),
        })
    }
    
    pub fn get_datetime(&self) -> &NaiveDateTime {
        &self.datetime
    }

    pub fn get_agent(&self) -> &str {
        &self.agent
    }

    pub fn get_amount(&self) -> u32 {
        self.amount
    }

    pub fn get_action(&self) -> &Operator {
        &self.action
    }

    pub fn get_comment(&self) -> &str {
        &self.comment
    }
}

/* struct AgoranEntity<'a> {
   name: &'a str,
   balances: HashMap<&'a Currency, u32>,
   } */
