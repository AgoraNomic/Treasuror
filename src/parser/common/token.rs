use std::iter::Peekable;
use std::str::CharIndices;

use chrono::naive::NaiveTime;

use super::operator::Operator;

macro_rules! produce_until {
    ( $cond:expr; $pt:pat in $iter:expr; $prod:expr; ) => {{
        let mut tmp_result = None;
        while let Some($pt) = $iter.next() {
            if $cond {
                tmp_result = Some($prod);
                break;
            }
        }
        tmp_result
    }};
}

macro_rules! produce_while {
    ( $cond:expr; $pt:pat in $iter:expr; $prod:expr; ) => {{
        let mut tmp_result = None;
        while let Some($pt) = $iter.peek().copied() {
            if !$cond {
                tmp_result = Some($prod);
                break;
            }
            $iter.next();
        }
        tmp_result
    }};
}

fn is_id_char(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '.' || c == '_' || c == '&'
}

pub struct TokenIterator<'a> {
    source: &'a str,
    chars: Peekable<CharIndices<'a>>,
}

impl<'a> TokenIterator<'a> {
    pub fn with_source(s: &'a str) -> TokenIterator<'a> {
        TokenIterator {
            source: s,
            chars: s.char_indices().peekable(),
        }
    }
}

impl<'a> Iterator for TokenIterator<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        let mut fidx: Option<usize> = None;
        let mut fchar: Option<char> = None;

        for (i, c) in self.chars.by_ref() {
            // println!("searching: {}", c);
            if c.is_whitespace() {
                continue;
            }
            // println!("found    : {}", c);
            fidx = Some(i);
            fchar = Some(c);
            break;
        }

        if let (Some(fi), Some(fc)) = (fidx, fchar) {
            // is a time; does not end until ']'
            if fc == '[' {
                produce_until!(
                    c == ']';
                    (i, c) in self.chars;
                    Token::Time(
                        NaiveTime::parse_from_str(&self.source[fi..i+1], "[%R]").unwrap()
                    );
                )
            // is an identifier; does not end until there are no more letters
            } else if fc.is_ascii_alphabetic() {
                produce_while!(
                    is_id_char(c);
                    (i, c) in self.chars;
                    Token::Identifier(String::from(&self.source[fi..i]));
                )
            // is an integer; does not end until there are no more numbers
            } else if fc.is_digit(10) {
                let first = produce_while!(
                    c.is_digit(10);
                    (i, c) in self.chars;
                    &self.source[fi..i];
                );

                if self.chars.peek().unwrap().1 == '.' {
                    //_or(&(0, ' ')).1 == '.' {
                    self.chars.next();
                    produce_while!(
                        c.is_digit(10);
                        (i, c) in self.chars;
                        Token::Float(self.source[fi..i].parse::<f32>().unwrap());
                    )
                } else {
                    Some(Token::Integer(first.unwrap().parse::<u32>().unwrap()))
                }
            // these are just single characters
            } else if fc == '*' {
                Some(Token::Blob)
            } else if fc == ':' {
                Some(Token::Separator)
            } else if fc == '+' {
                Some(Token::Op(Operator::Plus))
            } else if fc == '-' {
                Some(Token::Op(Operator::Minus))
            // transaction operator; takes an identifier
            } else if fc == '>' {
                produce_while!(
                    is_id_char(c);
                    (i, c) in self.chars;
                    Token::Op(Operator::Transfer(String::from(&self.source[fi+1..i])));
                )
            // strings end when there is a terminating '"'
            } else if fc == '"' {
                produce_until!(
                    c == '"';
                    (i, c) in self.chars;
                    Token::String(String::from(&self.source[fi+1..i]));
                )
            // command; takes an identifier
            } else if fc == '#' {
                produce_until!(
                    c.is_whitespace();
                    (i, c) in self.chars;
                    Token::Command(String::from(&self.source[fi+1..i]));
                )
            } else {
                println!("unknown char: {}", fc);
                None
            }
        } else {
            None
        }
    }
}

#[derive(Clone)]
pub enum Token {
    Time(NaiveTime),
    Identifier(String),
    Integer(u32),
    Blob,
    Separator,
    Float(f32),
    String(String),
    Op(Operator),
    Command(String),
}

impl Token {
    pub fn extract_float(&self) -> f32 {
        if let Token::Float(f) = self {
            *f
        } else {
            panic!("cannot extract float");
        }
    }

    pub fn extract_int(&self) -> u32 {
        if let Token::Integer(i) = self {
            *i
        } else {
            panic!("cannot extract int");
        }
    }

    pub fn extract_string(&self) -> &str {
        match self {
            Token::Identifier(s) | Token::String(s) | Token::Command(s) => s,
            _ => panic!("cannot extract string"),
        }
    }

    pub fn extract_operator(&self) -> Operator {
        if let Token::Op(o) = &self {
            o.clone()
        } else {
            panic!("cannot extract operator");
        }
    }
}
