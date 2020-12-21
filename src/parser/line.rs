#![allow(unused_assignments)]

use chrono::naive::{NaiveDate, NaiveDateTime};

use super::{Amount, Operator, Token, TokenIterator};

macro_rules! match_first_pop {
    ($v:ident { $( $t:pat => $b:block ),+, } else $e:block) => {{
        // println!("{}", $v.len());
        let tmp_first = $v.get(0).cloned();
        match tmp_first {
            $(Some($t) => {
                $v.remove(0);
                $b
            },)*
            Some(_) | None => $e,
        }
    }}
}

#[derive(Clone)]
pub struct Line<'a> {
    datetime: NaiveDateTime,
    action: Statement<'a>,
}

impl<'a> Line<'a> {
    pub fn with_date_from_str(date: &'a NaiveDate, ln: &'a mut str) -> Option<Line<'a>> {
        // if you'll remember we had to add whitespace to the end of every line
        // or else tokens would not work. now is when it would've been nice to
        // have a workaround for that detail.
        if ln.trim().is_empty() {
            return None;
        }
        let mut tokens: Vec<Token> = TokenIterator::from_str(ln).collect();

        Some(Line {
            datetime: match_first_pop!(tokens {
                Token::Time(t) => { date.and_time(t) },
            } else { date.and_hms(0,0,0) }),
            action: Statement::from_vec(tokens).expect("bad statement"),
        })
    }

    pub fn expand(self) -> Vec<Line<'a>> {
        match self.action {
            Statement::Transaction {..} => {
                let mut actions = self.action.expand();
                if actions.len() == 1 {
                    vec![Line { datetime: self.datetime, action: actions.remove(0) }]
                } else {
                    vec![
                        Line {
                            datetime: self.datetime,
                            action: actions.remove(0),
                        },
                        Line {
                            datetime: self.datetime,
                            action: actions.remove(0),
                        },
                    ]
                }
            },
            Statement::Command {..} => vec![
                Line {
                    datetime: self.datetime,
                    action: self.action.clone(),
                },
                Line {
                    datetime: self.datetime,
                    action: self.action,
                }
            ],
        }
    }
    
    pub fn get_datetime(&self) -> NaiveDateTime {
        self.datetime
    }

    pub fn get_action(&self) -> &Statement {
        &self.action
    }
}

#[derive(Clone)]
pub enum Statement<'a> {
    Transaction {
        agent: String,
        amount: Amount,
        operator: Operator<'a>,
        comment: String,
    },
    Command {
        cmd: String,
        args: Vec<Token<'a>>
    },
}

impl<'a> Statement<'a> {
    pub fn from_vec(mut tokens: Vec<Token<'a>>) -> Option<Statement<'a>> {
        match tokens[0] {
            Token::Identifier(i) => Some(Statement::Transaction {
                agent: match_first_pop!(tokens {
                    Token::Identifier(_) => { String::from(i) },
                } else { return None }),
                amount: Amount::from_vec(&mut tokens),
                operator: match_first_pop!(tokens {
                    Token::Op(o) => { o },
                } else { return None }), 
                comment: match_first_pop!(tokens {
                    Token::String(s) => { String::from(s) },
                } else { String::from("") }),
            }),
            Token::Command(c) => Some(Statement::Command {
                cmd: String::from(c),
                args: tokens,
            }),
            _ => None
        }
    }

    pub fn expand(self) -> Vec<Statement<'a>> {
        match self {
            Statement::Transaction {
                agent: agt,
                amount: amt,
                operator: op,
                comment: cmt,
            } => match op {
                Operator::Plus | Operator::Minus => vec![
                    Statement::Transaction {
                        agent: agt,
                        amount: amt,
                        operator: op,
                        comment: cmt,
                    }
                ],
                Operator::Transfer(s) => vec![
                    Statement::Transaction {
                        agent: agt.clone(),
                        amount: amt,
                        operator: Operator::Minus,
                        comment: format!(
                            "Transfer {}{}",
                            s,
                            if cmt == "" {
                            String::from("")
                            } else {
                            String::from(": ") + &cmt
                            }),
                    },
                    Statement::Transaction {
                        agent: String::from(s),
                        amount: amt,
                        operator: Operator::Plus,
                        comment: format!(
                            "Transfer {}{}",
                            agt,
                            if cmt == "" {
                                String::from("") 
                            } else {
                                String::from(": ") + &cmt
                            }),
                    }
                ],
            },
            Statement::Command {..} => vec![self],
        }
    }
   
    pub fn get_agent(&self) -> Option<&str> {
        if let Statement::Transaction { agent: agt, .. } = self {
            Some(agt)
        } else {
            None
        }
    }

    pub fn get_amount(&self) -> Option<Amount> {
        if let Statement::Transaction { amount: amt, .. } = self {
            Some(*amt)
        } else {
            None
        }
    }

    pub fn get_operator(&self) -> Option<Operator> {
        if let Statement::Transaction { operator: op, .. } = self {
            Some(*op)
        } else {
            None
        }
    }

    pub fn get_comment(&self) -> Option<&str> {
        if let Statement::Transaction { comment: cmt, .. } = self {
            Some(cmt)
        } else {
            None
        }
    }
}

/* struct AgoranEntity<'a> {
   name: &'a str,
   balances: HashMap<&'a Currency, u32>,
   } */
