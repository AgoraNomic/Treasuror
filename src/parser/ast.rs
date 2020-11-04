use super::token::Token;

#[derive(Copy, Clone)]
pub enum Amount {
    Everything,
    AllOf(Currency),
    PartOf(Currency, u32),
}

impl Amount {
    pub fn from_vec(s: &mut Vec<Token>) -> Amount {
        match s[0] {
            Token::Integer(i) => {
                s.remove(0);
                Amount::PartOf(
                    Currency::from_str(s.remove(0).extract_string()).unwrap(),
                    i)
            },
            Token::Blob => match s[1] {
                Token::Identifier(i) => {
                    s.remove(1);
                    s.remove(0);
                    Amount::AllOf(Currency::from_str(&i).unwrap())
                },
                _ => {
                    s.remove(0);
                    Amount::Everything
                },
            },
            _ => panic!("invalid token"),
        }
    }

    pub fn pretty(&self) -> String {
        match self {
            Amount::Everything => String::from("everything"),
            Amount::AllOf(c) => String::from("all of ") + c.abbr(),
            Amount::PartOf(c, a) => a.to_string() + c.abbr(),
        }
    }
}

#[derive(Copy, Clone)]
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

impl Currency {
    pub fn from_str(s: &str) -> Option<Currency> {
        match s {
            "cn" => Some(Currency::Coin),
            "wc" => Some(Currency::WinCard),
            "jc" => Some(Currency::JusticeCard),
            "lc" => Some(Currency::LegiCard),
            "vc" => Some(Currency::VoteCard),
            "wp" => Some(Currency::WinPoint),
            "bg" => Some(Currency::BlotBGone),
            "pd" => Some(Currency::Pendant),
            "xv" => Some(Currency::ExtraVote),
            _ => None,
        }
    }

    pub fn abbr(&self) -> &str {
        match self {
            Currency::Coin => "cn",
            Currency::WinCard => "wc",
            Currency::JusticeCard => "jc",
            Currency::LegiCard => "lc",
            Currency::VoteCard => "vc",
            Currency::WinPoint => "wp",
            Currency::BlotBGone => "bg",
            Currency::Pendant => "pd",
            Currency::ExtraVote => "xv",
        }
    }
}

#[derive(Copy, Clone)]
pub enum Operator<'a> {
    Plus,
    Minus,
    Transfer(&'a str),
}
