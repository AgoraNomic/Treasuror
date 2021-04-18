use super::token::Token;

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
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
    pub fn from_abbr(s: &str) -> Option<Currency> {
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

    pub fn is_card(&self) -> bool {
        *self == Currency::WinCard
            || *self == Currency::JusticeCard
            || *self == Currency::LegiCard
            || *self == Currency::VoteCard
    }
}

#[derive(Copy, Clone)]
pub enum FullUnit {
    Bare(Currency),
    Boatload(Currency),
}

impl FullUnit {
    pub fn from_vec(s: &mut Vec<Token>) -> FullUnit {
        if s.is_empty() {
            panic!("no valid unit");
        }

        if let Token::Identifier(i1) = s[0].clone() {
            s.remove(0);

            if s.len() >= 2 {
                if let (Token::Separator, Token::Identifier(i2)) = (s[0].clone(), s[1].clone()) {
                    s.remove(0);
                    s.remove(0);
                    if i1 == "bl" {
                        FullUnit::Boatload(Currency::from_abbr(&i2).unwrap())
                    } else {
                        panic!("invalid unit prefix!");
                    }
                } else {
                    FullUnit::Bare(
                        Currency::from_abbr(&i1)
                            .unwrap_or_else(|| panic!("invalid currency: {}", i1)),
                    )
                }
            } else {
                FullUnit::Bare(Currency::from_abbr(&i1).unwrap())
            }
        } else {
            panic!("no valid unit given");
        }
    }

    pub fn currency(&self) -> Currency {
        match self {
            FullUnit::Bare(c) => *c,
            FullUnit::Boatload(c) => *c,
        }
    }
}
