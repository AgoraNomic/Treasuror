use std::collections::HashMap;

use crate::{
    model::{Entity, EntityKind, Inventory},
    parser::{Amount, Currency, FullUnit, Operator, Transaction},
};

pub struct Context {
    entities: HashMap<String, Entity>,
    flotation: f32,
}

impl Context {
    pub fn new() -> Context {
        Context {
            entities: HashMap::new(),
            flotation: 5.0,
        }
    }

    pub fn boatloads(&self, amt: f32) -> u32 {
        (self.flotation * amt).ceil() as u32
    }

    pub fn apply(&mut self, trans: &Transaction) {
        for t in trans.expand() {
            let np = self.new_player(t.agent().to_string());
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

            let player = self.entities.entry(t.agent().to_string()).or_insert(np);

            match t.operator() {
                Operator::Plus => player.grant(currency, amount),
                Operator::Minus => player.revoke(currency, amount),
                _ => panic!("transfer should not appear here"),
            };
        }
    }

    pub fn new_player(&self, name: String) -> Entity {
        Entity::new(
            name,
            EntityKind::Player,
            self.default_map(EntityKind::Player),
        )
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
}
