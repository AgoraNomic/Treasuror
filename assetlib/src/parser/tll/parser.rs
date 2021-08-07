use std::io::{prelude::*, Chain};

use chrono::naive::NaiveDate;

use crate::parser::tll::Line;

pub struct Parser<R: BufRead> {
    reader: R,
    date: Option<NaiveDate>,
}

impl<R: BufRead> Parser<R> {
    pub fn from_reader(reader: R) -> Parser<R> {
        Parser { reader, date: None }
    }

    pub fn chain<S: BufRead>(self, next: S) -> Parser<Chain<R, S>> {
        Parser {
            reader: self.reader.chain(next),
            date: self.date,
        }
    }

    pub fn next_raw(&mut self) -> Option<Line> {
        let mut text = String::new();
        match self.reader.read_line(&mut text) {
            Ok(0) => None,
            Ok(_) => {
                if let Some(date) = self.date {
                    Line::with_date_from_str(date, &mut text).or_else(|| {
                        self.date = None;
                        self.next_raw()
                    })
                } else if text.is_empty() {
                    self.next_raw()
                } else if let Ok(date) = NaiveDate::parse_from_str(text.trim(), "%F") {
                    self.date = Some(date);
                    self.next_raw()
                } else {
                    eprintln!("got here somehow");
                    eprintln!("{}", text);
                    self.next_raw()
                }
            }
            Err(e) => panic!("Problem reading file: {}", e),
        }
    }
}
