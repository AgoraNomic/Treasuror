use std::str::FromStr;

pub enum Ministry {
    Compliance,
    Legislation,
    Economy,
    Legacy,
    Participation,
}

impl Ministry {
    fn from_abbr(s: &str) -> Option<Ministry> {
        match &s.to_lowercase()[..] {
            "c" | "co" | "com" | "compliance" =>
                Some(Ministry::Compliance),
            "l" | "legis" | "legislation" =>
                Some(Ministry::Legislation),
            "e" | "eco" | "econ" | "economy" =>
                Some(Ministry::Economy),
            "w" | "win" | "legacy" =>
                Some(Ministry::Legacy),
            "v" | "voting" | "p" | "par" | "part" | "participation" =>
                Some(Ministry::Participation),
            _ => None,
        }
    }
}
