use std::io::prelude::*;

use crate::parser::gsdl::Directive;

pub struct Parser<R: BufRead> {
    reader: R,
    linum: u32,
}

impl<R: BufRead> Parser<R> {
    pub fn from_reader(reader: R) -> Parser<R> {
        Parser { reader, linum: 0 }
    }

    pub fn next_raw(&mut self) -> Option<Directive> {
        let mut text = String::new();
        match self.reader.read_line(&mut text) {
            Ok(0) => None,
            Ok(_) => {
                self.linum += 1;
                if text.trim().is_empty() {
                    self.next_raw()
                } else {
                    Some(Directive::with_source(&text).unwrap_or_else(|e| {
                        panic!("L{}: {:?}", self.linum, e);
                    }))
                }
            }
            Err(e) => panic!("Problem reading file: {}", e),
        }
    }
}
