use std::collections::HashMap;

use crate::{
    parser::{Operator, Currency, Statement},
    model::{Entity, EntityKind},
};

pub struct Context {
    entities: HashMap<String, Inventory>,
    flotation: f32,
}

impl Context {
    pub fn new() -> Context {
        Context {
            entities: Vec::new(),
            flotation: 1.0,
        }
    }

    pub fn boatloads(&self, amt: f32) -> u32 {
        (self.flotation * amt).ciel() as u32
    }

    pub fn apply(&mut self, trans: Transaction) {
        for t in trans.expand() {
            let mut player = entities.entry(t.agent().clone())
                .or_insert(self.new_player(t.agent().clone()));

            let (currency, amount) = match t.amount() {
                Amount::PartOf(unit, amt) => {
                    match unit {
                        Bare(c) => (c, amt),
                        Boatload(c) => (c, self.boatloads(amt as f32)),
                    }
                },
                Amount::AllOf(c) => (c, player.balance(c)),
                Amount::Everything => {
                    eprintln!("everything not implemented!");
                    (Currency::Coin, 0)
                },
            }

            match t.operator() {
                Operator::Plus => player.grant_raw(
        }
    }

    pub fn new_player(&mut self, name: String) -> Entity {
        Entity {
            name: name,
            kind: EntityKind::Player,
            inventory: self.default_map(EntityKind::Player),
        }
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
                let m = HashMap::with_capacity(5);
                m.insert(Currency::Coin, self.boatloads(10));
                m.insert(Currency::WinCard, 1);
                m.insert(Currency::JusticeCard, 1);
                m.insert(Currency::LegiCard, 1);
                m.insert(Currency::VoteCard, 1);
                m
            },
            _ => HashMap::with_capacity(1),
        }
    }
}

