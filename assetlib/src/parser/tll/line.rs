#![allow(unused_assignments)]

use chrono::naive::{NaiveDate, NaiveDateTime};

use crate::parser::{
    common::{token_com::*, TokenIterator},
    error::*,
};

use super::{error::*, Command, Transaction};

#[derive(Clone)]
pub struct Line {
    datetime: NaiveDateTime,
    action: Command,
}

impl Line {
    pub fn with_date_from_str(date: NaiveDate, ln: &str) -> Result<Line, AnyError<&str>> {
        if ln.trim().is_empty() {
            return Err(AnyError::Syntax(SyntaxError {
                message: "".to_string(),
                kind: ErrorKind::Empty,
            }));
        }

        let mut tokens = Vec::new();

        for tr in TokenIterator::with_source(ln) {
            tokens.push(tr?);
        }

        Ok(Line {
            datetime: if let Ok(t) = expect_time(&mut tokens, "") {
                date.and_time(t)
            } else {
                date.and_hms(0, 0, 0)
            },
            action: if let Ok(c) = expect_command(&mut tokens, "") {
                Command::from_name_and_vec(c.to_string(), tokens)?
            } else {
                Command::Transaction(Transaction::from_vec(tokens)?)
            },
        })
    }

    pub fn datetime(&self) -> NaiveDateTime {
        self.datetime
    }

    pub fn action(&self) -> &Command {
        &self.action
    }
}
