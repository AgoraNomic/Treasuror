use chrono::naive::{NaiveDate, NaiveDateTime};

use crate::token::{Token, TokenIterator};

macro_rules! match_increment {
    ($v:ident in $i:ident { $( $t:pat => $b:block ),+, } else $e:block) => { match $v {
        $(
            Some($t) => {
                $v = $i.next();
                $b
            },)*
        Some(_) => $e,
        None => $e,
    }}
}

pub struct Transaction/*<'a>*/ {
    datetime: NaiveDateTime,
    agent: String,
    amount: u32,
/*    agent: &'a AgoranEntity,
    action: Operator,
    comment: &'a str, */
}

impl Transaction {
    pub fn with_date_from_str(date: &NaiveDate, ln: String) -> Option<Transaction> {
        if ln.is_empty() {
            return None;
        }
        let mut tokens = TokenIterator::from_str(&ln);
        let mut current_token = tokens.next();

        Some(Transaction {
            datetime: match_increment!(current_token in tokens {
                Token::Time(t) => { date.and_time(t) },
            } else { date.and_hms(0,0,0) }),
            agent: match_increment!(current_token in tokens {
                Token::Identifier(i) => { i },
            } else { String::from("?????") }),
            amount: match_increment!(current_token in tokens {
                Token::Integer(i) => { i },
                Token::Blob => { 10000 },
            } else { 0 }),
        })
    }
    
    pub fn get_datetime(&self) -> &NaiveDateTime {
        &self.datetime
    }

    pub fn get_agent(&self) -> &str {
        &self.agent
    }

    pub fn get_amount(&self) -> u32 {
        self.amount
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
