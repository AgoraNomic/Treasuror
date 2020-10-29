use chrono::naive::{NaiveDate, NaiveDateTime};

use crate::token::{Token, TokenIterator};

pub struct Transaction {
    datetime: NaiveDateTime,
/*    amount: u32,
    agent: &'a AgoranEntity,
    action: Operator,
    comment: &'a str, */
}

impl Transaction {
    pub fn with_date_from_str(date: &NaiveDate, ln: String) -> Option<Transaction> {
        let mut tokens = TokenIterator::from_str(&ln);
        let current_token = match tokens.next() {
            Some(t) => t,
            None => { return None }
        };

        let dt = match current_token {
            Token::Time(t) => date.and_time(t),
            _ => date.and_hms(0,0,0),
        };

        Some(Transaction {
            datetime: dt,
        })
    }
    
    pub fn get_datetime(&self) -> &NaiveDateTime {
        &self.datetime
    }
}

/* struct AgoranEntity<'a> {
    name: &'a str,
    balances: HashMap<&'a Currency, u32>,
}

enum Currency {
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

enum Operator {
    Plus,
    Minus,
} */
