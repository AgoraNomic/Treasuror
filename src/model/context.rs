use std::collections::HashMap;

use chrono::naive::{NaiveDateTime, MIN_DATE};

use crate::{
    match_first_pop,
    model::{Entity},
    parser::{
        ast::{Amount, Currency, FullUnit, Operator, Token},
        tll::{Command, Line},
    },
};

pub struct Context {
    entities: HashMap<String, Entity>,
    flotation: f32,
    datetime: NaiveDateTime,
}

impl Context {
    pub fn new() -> Context {
        Context {
            entities: HashMap::new(),
            flotation: 1.0,
            datetime: MIN_DATE.and_hms(0, 0, 0),
        }
    }

    pub fn relevel(&mut self) {
        self.flotation = self
            .entities
            .values()
            .map(|ent| ent.balance(Currency::Coin))
            .sum::<u32>() as f32
            / 2500.0;
    }

    pub fn apply(&mut self, line: &Line) {
        if self.verify_datetime(line.datetime()) {
            self.datetime = line.datetime();
        } else {
            panic!(
                "traveling back in time! {} is before {}.",
                line.datetime().format("%F %R"),
                self.datetime.format("%F %R")
            );
        }

        let trans = line
            .action()
            .transaction()
            .expect("cannot apply non-transaction");

        for t in trans.expand() {
            let (currency, amount) = match t.amount() {
                Amount::PartOf(unit, amt) => match unit {
                    FullUnit::Bare(c) => (c, amt),
                    FullUnit::Boatload(c) => (c, self.boatloads(amt as f32)),
                },
                Amount::AllOf(c) => (c, u32::MAX),
                Amount::Everything => {
                    eprintln!("everything not implemented!");
                    (Currency::Coin, 0)
                }
            };

            let player = self
                .entities
                .get_mut(t.agent())
                .expect(&format!("no such entity: {}", t.agent()));

            match t.operator() {
                Operator::Plus => player.grant(currency, amount),
                Operator::Minus => player.revoke(currency, amount),
                _ => panic!("transfer should not appear here"),
            };
        }
    }

    pub fn exec(&mut self, com: &Command) {
        match com.command() {
            "relevel" => self.relevel(),
            "newplayer" => {
                let mut args = com.args().clone();
                args.remove(0);
                let identifier = match_first_pop!(args {
                    Token::Identifier(s) => { s },
                } else { panic!("expected identifier in #newplayer command") });

                let full_name = match_first_pop!(args {
                    Token::String(s) => { s },
                    Token::Identifier(s) => { s },
                } else { identifier.clone() });

                self.add_player(identifier, full_name);
            }
            _ => eprintln!("no such command: {}", &com.command()),
        }
    }

    pub fn add_player(&mut self, identifier: String, full_name: String) {
        if self.entities.get(&full_name).is_some() {
            panic!("entity already exists: {}", full_name);
        }

        self.entities.insert(
            identifier.to_string(),
            Entity::player(identifier, full_name)
        );
    }

    pub fn display(&self) -> String {
        let mut result = String::new();
        for (name, ent) in self.entities.iter() {
            result.push_str(name);
            result.push_str(": ");
            for (curr, amount) in ent.inventory().iter() {
                result.push_str(&amount.to_string());
                result.push_str(curr.abbr());
                result.push_str(", ");
            }
            result.push('\n');
        }
        result
    }

    pub fn boatloads(&self, amt: f32) -> u32 {
        (self.flotation * amt).ceil() as u32
    }

    pub fn verify_datetime(&self, other: NaiveDateTime) -> bool {
        other >= self.datetime
    }
}
