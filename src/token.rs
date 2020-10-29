use chrono::naive::NaiveTime;

use std::str::CharIndices;

/* macro_rules! produce_while {
    ( $p:expr; $v:pat in $s:stmt; $b:expr ) => {
        let tmp_result = None;
        while let Some($v) = $s {
            if $b {
                tmp_result = Some($p);
                break;
            }
        }
        tmp_result
    }
} */

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
        
        while let Some((i, c)) = self.chars.next() {
            if c.is_whitespace() {
                continue;
            }
            fidx = Some(i);
            fchar = Some(c);
            break;
        }

//        println!("i: {}, c: '{}'", fidx.expect("no i"), fchar.expect("no c"));

        let mut result: Option<Self::Item> = None;

        if let (Some(fi), Some(fc)) = (fidx, fchar) {
            if fc == '[' {
                println!("found time!");
                while let Some((i, c)) = self.chars.next() {
                    if c == ']' {
                        result = Some(
                            Token::Time(
                                NaiveTime::parse_from_str(&self.source[fi..i+1], "[%R]").unwrap()
                                )
                            );
                        break;
                    }
                }
            } else if fc.is_ascii_alphabetic() {
                println!("found id!");
                while let Some((i, c)) = self.chars.next() {
                    if !(c.is_ascii_alphabetic() || c.is_digit(10)) {
                        result = Some(Token::Identifier(String::from(&self.source[fi..i])));
                        break;
                    }
                }
            } else if fc.is_digit(10) {
                println!("found number!");
                while let Some((i, c)) = self.chars.next() {
                    if !c.is_digit(10) {
                        result = Some(
                            Token::Integer(self.source[fi..i].parse::<u32>().unwrap())
                            );
                        break;
                    }
                }
            } else if fc == '*' {
                println!("found blob!");
                result = Some(Token::Blob);
            } else if fc == '+' {
                println!("found plus!");
                result = Some(Token::Op(Operator::Plus));
            } else if fc == '-' {
                println!("found minus!");
                result = Some(Token::Op(Operator::Minus));
            } else if fc == '>' {
                println!("found transfer!");
                while let Some((i, c)) = self.chars.next() {
                    if !(c.is_ascii_alphabetic() || c.is_digit(10)) {
                        result = Some(Token::Op(Operator::Transfer(String::from(&self.source[fi+1..i]))));
                        break;
                    }
                }
            }
        }
        return result;
    }
}

pub enum Token<'a> {
    Time(NaiveTime),
    Identifier(String),
    Integer(u32),
    Blob,
    Float(f32),
    String(&'a str),
    Op(Operator),
    Command(&'a str),
}

pub enum Operator {
    Plus,
    Minus,
    Transfer(String),
}
