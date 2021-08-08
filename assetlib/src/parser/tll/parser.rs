use std::io::prelude::*;

use chrono::naive::NaiveDate;

use crate::parser::{
    error::*,
    tll::Line
};

pub struct Parser<R: BufRead> {
    reader: R,
    date: Option<NaiveDate>,
    linum: u32,
}

impl<R: BufRead> Parser<R> {
    pub fn from_reader(reader: R) -> Parser<R> {
        Parser { reader, date: None, linum: 0 }
    }

    pub fn next_raw(&mut self) -> Option<Line> {
        let mut text = String::new();
        match self.reader.read_line(&mut text) {
            Ok(0) => None,
            Ok(_) => {
                self.linum += 1;

                if text.trim().is_empty() {
                    self.date = None;
                    self.next_raw()
                } else if let Some(date) = self.date {
                    Some(
                        Line::with_date_from_str(date, &text)
                            .unwrap_or_else(|e| panic!("L{}: {:?}", self.linum, e))
                    )
                } else if let Ok(date) = NaiveDate::parse_from_str(text.trim(), "%F") {
                    self.date = Some(date);
                    self.next_raw()
                } else {
                    eprintln!("got here somehow");
                    eprintln!("{}", text);
                    self.next_raw()
                }
            },
            Err(e) => panic!("Problem reading file: {}", e),
        }
    }
}
