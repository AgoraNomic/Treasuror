#![allow(unused_assignments)]

use chrono::naive::{NaiveDate, NaiveDateTime};

use super::ast::{
    amount::Amount,
    operator::Operator,
    token::{Token, TokenIterator},
};

macro_rules! match_first_pop {
    ($v:ident { $( $t:pat => $b:block ),+, } else $e:block) => {{
        let tmp_first = $v.get(0).cloned();
        match tmp_first {
            $(Some($t) => {
                $v.remove(0);
                $b
            },)*
            Some(_) => $e
            None => $e,
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
    pub fn with_date_from_str(date: &'a NaiveDate, ln: &'a mut str) -> Option<Transaction<'a>> {
        if ln.is_empty() {
            return None;
        }
        let mut tokens: Vec<Token> = TokenIterator::from_str(ln).collect();

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

    pub fn expand_transfer(self) -> Vec<Transaction<'a>> {
        let result: Vec<Transaction> = Vec::new();
        match self.action {
            Operator::Plus | Operator::Minus => vec![self],
            Operator::Transfer(s) => vec![
                Transaction {
                   datetime: self.datetime.clone(),
                   agent: self.agent.clone(),
                   amount: self.amount,
                   action: Operator::Minus,
                   comment: format!("Transfer {}{}",
                                    s,
                                    if self.comment == "" {
                                        String::from("")
                                    } else {
                                        String::from(": ") + &self.comment
                                    }),
                },
                Transaction {
                   datetime: self.datetime,
                   agent: String::from(s),
                   amount: self.amount,
                   action: Operator::Plus,
                   comment: format!("Transfer {}{}",
                                    self.agent,
                                    if self.comment == "" { String::from("") } else {
                                        String::from(": ") + &self.comment
                                    }),
                }
            ],
        }
    }
}

/* struct AgoranEntity<'a> {
   name: &'a str,
   balances: HashMap<&'a Currency, u32>,
   } */
