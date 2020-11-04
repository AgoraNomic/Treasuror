use super::token::Token;

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
