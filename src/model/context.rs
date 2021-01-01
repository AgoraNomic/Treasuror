use std::collections::HashMap;

use chrono::naive::{NaiveDateTime, MIN_DATE};

use crate::{
    model::{Entity, EntityKind, Inventory},
    parser::{Amount, Command, Currency, FullUnit, Line, Operator},
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
            .expect("cannot apply transaction");

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
            "newplayer" => self.new_player(String::from(com.args()[1].extract_string())),
            _ => eprintln!("no such command: {}", com.command()),
        }
    }

    pub fn new_player(&mut self, name: String) {
        if self.entities.get(&name).is_some() {
            panic!("entity already exists: {}", name);
        }

        self.entities.insert(
            name.clone(),
            Entity::new(
                name,
                EntityKind::Player,
                self.default_map(EntityKind::Player),
            ),
        );
    }

    /// Meant to be a better way of allocating maps for different kinds of
    /// entities. Players start with coins and cards so they for sure need
    /// five inventory spots. Other entities are usually only used to store
    /// coins. This isn't perfect, but it should be marginally better than
    /// leaving it up to default allocation.
    ///
    /// If assets change enough to warrant it, this may need to be updated
    /// at some point. In that case, we will probably want to change
    /// allocation rules based on the date the entity joined.
    pub fn default_map(&self, ek: EntityKind) -> Inventory {
        match ek {
            EntityKind::Player => {
                let mut m = HashMap::with_capacity(5);
                m.insert(Currency::Coin, self.boatloads(10.0));
                m.insert(Currency::WinCard, 1);
                m.insert(Currency::JusticeCard, 1);
                m.insert(Currency::LegiCard, 1);
                m.insert(Currency::VoteCard, 1);
                m
            }
            _ => HashMap::with_capacity(1),
        }
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
