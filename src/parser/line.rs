#![allow(unused_assignments)]

use chrono::naive::{NaiveDate, NaiveDateTime};

use super::{Token, TokenIterator, Statement};

#[macro_export]
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

/*    pub fn expand(self) -> Vec<Line<'a>> {
        match self.action {
            Statement::Transaction(t) => {
//                let mut actions = self.get_action().get_transaction().unwrap().expand();
                
                t.clone().expand().iter().cloned().map(|w| Line {
                    datetime: self.datetime,
                    action: Statement::Transaction(w),
                }).collect()

//                if actions.len() == 1 {
//                    vec![
//                        Line {
//                            datetime: self.datetime,
//                            action: Statement::Transaction(actions.remove(0))
//                        }
//                    ]
//                } else {
//                    vec![
//                        Line {
//                            datetime: self.datetime,
//                            action: Statement::Transaction(actions.remove(0)),
//                        },
//                        Line {
//                            datetime: self.datetime,
//                            action: Statement::Transaction(actions.remove(0)),
//                        },
//                    ]
//                }
            },
            _ => vec![
                Line {
                    datetime: self.datetime,
                    action: self.action.clone(),
                }
            ],
        }
    } */
    
    pub fn get_datetime(&self) -> NaiveDateTime {
        self.datetime
    }

    pub fn get_action(&self) -> &Statement {
        &self.action
    }
}

