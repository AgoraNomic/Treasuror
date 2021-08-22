use super::unit::{Currency, FullUnit};

#[derive(Copy, Clone)]
pub enum Amount {
    Everything,
    AllOf(Currency),
    PartOf(FullUnit, u32),
}

impl Amount {
    pub fn pretty(&self) -> String {
        match self {
            Amount::Everything => String::from("everything"),
            Amount::AllOf(c) => String::from("all of ") + c.abbr(),
            Amount::PartOf(c, a) => {
                a.to_string()
                    + match c {
                        FullUnit::Bare(_) => "",
                        FullUnit::Boatload(_) => "bl:",
                    }
                    + c.currency().abbr()
            }
        }
    }
}
