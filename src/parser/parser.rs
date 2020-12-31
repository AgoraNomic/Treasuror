use std::fs::File;
use std::io::{BufRead, BufReader, Lines, Result as IoResult};
use std::path::Path;

use chrono::naive::NaiveDate;

use crate::parser::Line as ParserLine;

pub struct Parser {
    iterator: Lines<BufReader<File>>,
    date: Option<NaiveDate>,
}

impl Parser {
    pub fn from_filename<P>(filename: P) -> IoResult<Parser>
    where
        P: AsRef<Path>,
    {
        let file = File::open(filename)?;
        Ok(Parser {
            iterator: BufReader::new(file).lines(),
            date: None,
        })
    }

    pub fn next_raw(&mut self) -> Option<ParserLine> {
        match self.iterator.next() {
            Some(Ok(mut text)) => {
                if let Some(date) = self.date {
                    // i'm not sure why but token production only works if there is
                    // a whitespace at the end. i tried to find a workaround but
                    // i'm too tired for this so here you go.
                    text.push('\n');
                    match ParserLine::with_date_from_str(date, &mut text) {
                        Some(l) => Some(l),
                        None => {
                            self.date = None;
                            self.next_raw()
                        }
                    }
                } else if text.is_empty() {
                    self.next_raw()
                } else if let Ok(date) = NaiveDate::parse_from_str(&text, "%F") {
                    self.date = Some(date);
                    self.next_raw()
                } else {
                    eprintln!("got here somehow");
                    self.next_raw()
                }
            }
            Some(Err(e)) => panic!(format!("problem reading file: {}", e)),
            None => None,
        }
    }
}
