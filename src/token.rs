use std::iter::Peekable;
use std::str::CharIndices;

use chrono::naive::NaiveTime;

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
    }}
}

macro_rules! produce_while {
    ( $cond:expr; $pt:pat in $iter:expr; $prod:expr; ) => {{
        let mut tmp_result = None;
        while let Some($pt) = $iter.peek() {
            if !$cond {
                tmp_result = Some($prod);
                break;
            }
            $iter.next();
        }
        tmp_result
    }}
}
pub struct TokenIterator<'a> {
    source: &'a str,
    chars: Peekable<CharIndices<'a>>,
}

impl<'a> TokenIterator<'a> {
    pub fn from_str(s: &'a str) -> TokenIterator<'a> {
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
//            println!("searching: {}", c);
            if c.is_whitespace() {
                continue;
            }
//            println!("found    : {}", c);
            fidx = Some(i);
            fchar = Some(c);
            break;
        }

        if let (Some(fi), Some(fc)) = (fidx, fchar) {
            if fc == '[' {
                return produce_until!(
                    c == ']';
                    (i, c) in self.chars;
                    Token::Time(
                        NaiveTime::parse_from_str(&self.source[fi..i+1], "[%R]").unwrap()
                        );
                    );
            } else if fc.is_ascii_alphabetic() {
                let result = produce_while!(
                    c.is_ascii_alphabetic();
                    (i, c) in self.chars; 
                    &self.source[fi..*i];
                    );

//                println!("{}", result.unwrap());

                return Some(match result.expect("what did i do") {
                    "cn" => Token::Curr(Currency::Coin),
                    "wc" => Token::Curr(Currency::WinCard),
                    "jc" => Token::Curr(Currency::JusticeCard),
                    "lc" => Token::Curr(Currency::LegiCard),
                    "vc" => Token::Curr(Currency::VoteCard),
                    "wp" => Token::Curr(Currency::WinPoint),
                    "bg" => Token::Curr(Currency::BlotBGone),
                    "pd" => Token::Curr(Currency::Pendant),
                    "xv" => Token::Curr(Currency::ExtraVote),
                    nope => Token::Identifier(String::from(nope)),
                });
            } else if fc.is_digit(10) {
                return produce_while!(
                    c.is_digit(10);
                    (i, c) in self.chars; 
                    Token::Integer(self.source[fi..*i].parse::<u32>().unwrap());
                    );
            } else if fc == '*' {
                return Some(Token::Blob);
            } else if fc == '"' {
                return produce_until!(
                    c == '"';
                    (i, c) in self.chars;
                    Token::String(String::from(&self.source[fi+1..i]));
                    );
            } else if fc == '+' {
                return Some(Token::Op(Operator::Plus));
            } else if fc == '-' {
                return Some(Token::Op(Operator::Minus));
            } else if fc == '>' {
                return produce_while!(
                    c.is_ascii_alphabetic();
                    (i, c) in self.chars; 
                    Token::Op(Operator::Transfer(String::from(&self.source[fi+1..*i])));
                    );
            } else {
                println!("unknown char: {}", fc);
            }
        }
        return None;
    }
}

pub enum Token {
    Time(NaiveTime),
    Identifier(String),
    Curr(Currency),
    Integer(u32),
    Blob,
    Float(f32),
    String(String),
    Op(Operator),
    Command(String),
}

pub enum Operator {
    Plus,
    Minus,
    Transfer(String),
}


pub enum Currency {
    Coin,
    WinCard,
    JusticeCard,
    LegiCard,
    VoteCard,
    WinPoint,
    BlotBGone,
    Pendant,
    ExtraVote,
}
