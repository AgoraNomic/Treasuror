use std::io::{prelude::*, Chain};

use crate::parser::gsdl::Directive;

pub struct Parser<R: BufRead> {
    reader: R,
}

impl<R: BufRead> Parser<R> {
    pub fn from_reader(reader: R) -> Parser<R> {
        Parser { reader }
    }

    pub fn chain<S: BufRead>(self, next: S) -> Parser<Chain<R, S>> {
        Parser {
            reader: self.reader.chain(next),
        }
    }

    pub fn next_raw(&mut self) -> Option<Directive> {
        let mut text = String::new();
        match self.reader.read_line(&mut text) {
            Ok(0) => None,
            Ok(_) => {
                if text.trim().is_empty() {
                    self.next_raw()
                } else {
                    Directive::with_source(&text)
                }
            }
            Err(e) => panic!(format!("Problem reading file: {}", e)),
        }
    }
}
