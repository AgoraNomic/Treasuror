use chrono::naive::NaiveTime;

use std::str::CharIndices;

macro_rules! produce_until {
    ( $cond:expr; $pt:pat in $iter:expr; $prod:expr; ) => {{
        let mut tmp_result = None;
        {
            while let Some($pt) = $iter.next() {
                if $cond {
                    tmp_result = Some($prod);
                    break;
                }
            }
        }
        tmp_result
    }}
}

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
//                println!("found time!");
                result = produce_until!(
                    c == ']';
                    (i, c) in self.chars;
                    Token::Time(
                        NaiveTime::parse_from_str(&self.source[fi..i+1], "[%R]").unwrap()
                        );
                    );
            } else if fc.is_ascii_alphabetic() {
//                println!("found id!");
                result = produce_until!(
                    !(c.is_ascii_alphabetic() || c.is_digit(10));
                    (i, c) in self.chars; 
                    Token::Identifier(String::from(&self.source[fi..i]));
                    );
            } else if fc.is_digit(10) {
//                println!("found number!");
                result = produce_until!(
                    !c.is_digit(10);
                    (i, c) in self.chars; 
                    Token::Integer(self.source[fi..i].parse::<u32>().unwrap());
                    );
            } else if fc == '*' {
                result = Some(Token::Blob);
            } else if fc == '"' {
                result = produce_until!(
                    c == '"';
                    (i, c) in self.chars;
                    Token::String(&self.source[fi..i]);
                    );
            } else if fc == '+' {
                result = Some(Token::Op(Operator::Plus));
            } else if fc == '-' {
                result = Some(Token::Op(Operator::Minus));
            } else if fc == '>' {
                result = produce_until!(
                    !(c.is_ascii_alphabetic() || c.is_digit(10));
                    (i, c) in self.chars; 
                    Token::Op(Operator::Transfer(String::from(&self.source[fi+1..i])));
                    );
            } else {
                println!("unknown char: {}", fc);
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
