use crate::parser::{Currency};

struct Entity {
    name: String,
    kind: EntityKind,
    inventory: Inventory,
}

impl Entity {
    pub fn balance(&self, c: Currency) {
        self.inventory.get(c).unwrap_or(0)
    }

    pub fn grant_raw(&mut self, c: Currency, a: u32) {
        self.inventory.entry(c).or_insert(0) += a;
    }

    pub fn retract_raw(&mut self, c: Currency, a: u32) {
        let mut q = self.inventory.entry(c).or_insert(0);
        if q < a {
            panic!("attempt to retract below zero");
        } else {
            q += a;
        }
    }
}

#[derive(Clone, Copy)]
enum EntityKind {
    Player,
    Contract,
    Other,
}
