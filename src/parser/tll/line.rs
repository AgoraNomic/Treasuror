#![allow(unused_assignments)]

use chrono::naive::{NaiveDate, NaiveDateTime};

use super::Statement;
use crate::{
    match_first_pop,
    parser::common::{Token, TokenIterator},
};

#[derive(Clone)]
pub struct Line {
    datetime: NaiveDateTime,
    action: Statement,
}

impl Line {
    pub fn with_date_from_str(date: NaiveDate, ln: &mut str) -> Option<Line> {
        // if you'll remember we had to add whitespace to the end of every line
        // or else tokens would not work. now is when it would've been nice to
        // have a workaround for that detail.
        if ln.trim().is_empty() {
            return None;
        }
        let mut tokens: Vec<Token> = TokenIterator::with_source(ln).collect();

        Some(Line {
            datetime: match_first_pop!(tokens {
                Token::Time(t) => { date.and_time(t) },
            } else { date.and_hms(0,0,0) }),
            action: Statement::from_vec(tokens).expect("bad statement"),
        })
    }

    pub fn datetime(&self) -> NaiveDateTime {
        self.datetime
    }

    pub fn action(&self) -> &Statement {
        &self.action
    }
}
