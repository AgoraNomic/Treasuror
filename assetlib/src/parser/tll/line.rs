#![allow(unused_assignments)]

use crate::parser::tll::Command;
use crate::parser::tll::Transaction;
use chrono::naive::{NaiveDate, NaiveDateTime};

use crate::{
    match_first_pop,
    parser::common::{error::ParseError, Token, TokenIterator},
};

#[derive(Clone)]
pub struct Line {
    datetime: NaiveDateTime,
    action: Command,
}

impl Line {
    pub fn with_date_from_str(date: NaiveDate, ln: &mut str) -> Result<Line, ParseError<&str>> {
        if ln.trim().is_empty() {
            return None;
        }

        let mut tokens: Vec<Token> = TokenIterator::with_source(ln)
            .map(|t| t?)
            .collect();

        for tr in TokenIterator::with_source(ln) {
            match tr {
                Err(e) => { println!("{:?}", e); }
                _ => {}
            }
        }

        None

        // Some(Line {
        //     datetime: match_first_pop!(tokens {
        //         Token::Time(t) => { date.and_time(t) },
        //     } else { date.and_hms(0,0,0) }),
        //     action: if let Token::Command(c) = tokens[0].clone() {
        //         tokens.remove(0);
        //         Command::from_name_and_vec(c.to_string(), tokens).unwrap()
        //     } else {
        //         Command::Transaction(Transaction::from_vec(tokens).expect("no transaction"))
        //     },
        // })
    }

    pub fn datetime(&self) -> NaiveDateTime {
        self.datetime
    }

    pub fn action(&self) -> &Command {
        &self.action
    }
}
