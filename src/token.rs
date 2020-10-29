use chrono::naive::NaiveTime;

use std::str::CharIndices;

pub struct TokenIterator<'a> {
    source: &'a str,
    chars: CharIndices<'a>,
}

impl<'a> TokenIterator<'a> {
    pub fn from_str(s: &'a str) -> TokenIterator<'a> {
        TokenIterator {
            source: s,
            chars: s.char_indices(),
        }
    }
}

impl<'a> Iterator for TokenIterator<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Token<'a>> {
        let mut fidx: Option<usize> = None;
        let mut fchar: Option<char> = None;

        for (i, c) in self.chars.next() {
            if c.is_whitespace() {
                continue;
            }
            fidx = Some(i);
            fchar = Some(c);
        }

        let mut result: Option<Self::Item> = None;

        if let (Some(i), Some(c)) = (fidx, fchar) {
/*            if first.is_digit(10) {
                while let Some((i, c)) = self.chars.peek() {
                    if !c.is_digit(10) {
                        result = Some(Token::Integer(&s[fidx..idx+1].parse::<i32>().
                    }
                }
            } */

            if c == '[' {
                while let Some((i2, c2)) = self.chars.next() {
                    if c2 == ']' {
                        result = Some(
                            Token::Time(
                                NaiveTime::parse_from_str(&self.source[i..i2+1], "[%R]").unwrap()
                                )
                            );
                    }
                }
            }
        }
        return result;
    }
}

pub enum Token<'a> {
    Time(NaiveTime),
    Identifier(&'a str),
    Integer(i32),
    Float(f32),
    String(&'a str),
    Command(&'a str),
}
