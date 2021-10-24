#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum Currency {
    Coin,
    WinCard,
    JusticeCard,
    LegiCard,
    VoteCard,
    WinPoint,
    Winsome,
    BlotBGone,
    Pendant,
    ExtraVote,
    Votive,
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
            "ws" => Some(Currency::Winsome),
            "bg" => Some(Currency::BlotBGone),
            "pd" => Some(Currency::Pendant),
            "xv" => Some(Currency::ExtraVote),
            "vo" => Some(Currency::Votive),
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
            Currency::Winsome => "ws",
            Currency::BlotBGone => "bg",
            Currency::Pendant => "pd",
            Currency::ExtraVote => "xv",
            Currency::Votive => "vo",
        }
    }

    pub fn is_card(&self) -> bool {
        *self == Currency::WinCard
            || *self == Currency::JusticeCard
            || *self == Currency::LegiCard
            || *self == Currency::VoteCard
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum FullUnit {
    Bare(Currency),
    Boatload(Currency),
}

impl FullUnit {
    pub fn currency(&self) -> Currency {
        match self {
            FullUnit::Bare(c) => *c,
            FullUnit::Boatload(c) => *c,
        }
    }
}
