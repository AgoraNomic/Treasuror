use chrono::naive::{NaiveDate, NaiveTime, NaiveDateTime};

pub struct Transaction {
    datetime: NaiveDateTime,
/*    amount: u32,
    agent: &'a AgoranEntity,
    action: Operator,
    comment: &'a str, */
}

impl Transaction {
    pub fn with_date_from_str(date: &NaiveDate, ln: &str) -> Transaction {
        let mut words = ln.split_whitespace().peekable();

        Transaction {
            datetime: date.and_time(
                match NaiveTime::parse_from_str(words.peek().unwrap(), "[%R]") {
                    Ok(t) => {
                        words.next();
                        t
                    },
                    Err(_) => NaiveTime::from_hms(0, 0, 0),
                }),
        }
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
