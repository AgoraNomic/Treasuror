use chrono::naive::{NaiveDate, NaiveDateTime};

use crate::token::{Token, TokenIterator};

pub struct Transaction/*<'a>*/ {
    datetime: NaiveDateTime,
    agent: String
/*    amount: u32,
    agent: &'a AgoranEntity,
    action: Operator,
    comment: &'a str, */
}

impl Transaction {
    pub fn with_date_from_str(date: &NaiveDate, ln: String) -> Option<Transaction> {
        let mut tokens = TokenIterator::from_str(&ln);
        let mut current_token = tokens.next()?;

        let dt = match current_token {
            Token::Time(t) => {
                current_token = tokens.next()?;
                date.and_time(t)
            },
            _ => date.and_hms(0,0,0),
        };

        let amt = match current_token {
            Token::Identifier(i) => {
                current_token = tokens.next()?;
                i
            },
            _ => String::from("no one"),
        };

        Some(Transaction {
            datetime: dt,
            agent: amt,
        })
    }
    
    pub fn get_datetime(&self) -> &NaiveDateTime {
        &self.datetime
    }

    pub fn get_agent(&self) -> &str {
        &self.agent
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
