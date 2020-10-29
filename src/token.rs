use chrono::naive::NaiveTime;

use std::str::CharIndices;

macro_rules! produce_until {
    ( $p:expr; $v:pat in $s:expr; $b:expr ) => {{
        let mut tmp_result = None;
        {
            while let Some($v) = $s.next() {
                if $b {
                    tmp_result = Some($p);
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
                    Token::Time(
                        NaiveTime::parse_from_str(&self.source[fi..i+1], "[%R]").unwrap()
                        ); (i, c) in self.chars; c == ']'
                    );
            } else if fc.is_ascii_alphabetic() {
//                println!("found id!");
                result = produce_until!(
                    Token::Identifier(String::from(&self.source[fi..i]));
                    (i, c) in self.chars; !(c.is_ascii_alphabetic() || c.is_digit(10))
                    );
            } else if fc.is_digit(10) {
//                println!("found number!");
                result = produce_until!(
                    Token::Integer(self.source[fi..i].parse::<u32>().unwrap());
                    (i, c) in self.chars; !c.is_digit(10)
                    );
            } else if fc == '*' {
                result = Some(Token::Blob);
            } else if fc == '+' {
                result = Some(Token::Op(Operator::Plus));
            } else if fc == '-' {
                result = Some(Token::Op(Operator::Minus));
            } else if fc == '>' {
                result = produce_until!(
                    Token::Op(Operator::Transfer(String::from(&self.source[fi+1..i])));
                    (i, c) in self.chars; !(c.is_ascii_alphabetic() || c.is_digit(10))
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
