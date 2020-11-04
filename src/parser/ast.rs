use super::token::Token;

#[derive(Copy, Clone)]
pub enum Amount {
    Everything,
    AllOf(Currency),
    PartOf(FullUnit, u32),
}

impl Amount {
    pub fn from_vec(s: &mut Vec<Token>) -> Amount {
        match s[0] {
            Token::Integer(i) => {
                s.remove(0);
                Amount::PartOf(
                    FullUnit::from_vec(s), // s.remove(0).extract_string()).unwrap(),
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
            Amount::PartOf(c, a) => a.to_string() + match c {
                FullUnit::Bare(_) => "",
                FullUnit::Boatload(_) => "bl:"
            } + c.get_currency().abbr(),
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

#[derive(Copy, Clone)]
pub enum FullUnit {
    Bare(Currency),
    Boatload(Currency),
}

impl FullUnit {
    pub fn from_vec(s: &mut Vec<Token>) -> FullUnit {
        if s.len() < 1 {
            panic!("no valid unit");
        }

        if let Token::Identifier(i1) = s[0] {
            s.remove(0);

            if s.len() >= 2 {
                if let (Token::Separator, Token::Identifier(i2)) = (s[0], s[1]) {
                    s.remove(0);
                    s.remove(0);
                    if i1 == "bl" {
                        FullUnit::Boatload(Currency::from_str(i2).unwrap())
                    } else {
                        panic!("invalid unit prefix!");
                    }
                } else {
                    FullUnit::Bare(Currency::from_str(i1).unwrap())
                }
            } else {
                FullUnit::Bare(Currency::from_str(i1).unwrap())
            }
        } else {
            panic!("no valid unit given");
        }
    }

    pub fn get_currency(&self) -> Currency {
        match self {
            FullUnit::Bare(c) => *c,
            FullUnit::Boatload(c) => *c,
        }
    }
}
