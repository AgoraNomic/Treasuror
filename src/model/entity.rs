use crate::{model::Inventory, parser::Currency};

#[allow(dead_code)]
pub struct Entity {
    name: String,
    kind: EntityKind,
    inventory: Inventory,
}

impl Entity {
    pub fn new(name: String, kind: EntityKind, inventory: Inventory) -> Entity {
        Entity {
            name: name,
            kind: kind,
            inventory: inventory,
        }
    }

    pub fn balance(&self, c: Currency) -> u32 {
        *self.inventory.get(&c).unwrap_or(&0)
    }

    pub fn inventory(&self) -> &Inventory {
        &self.inventory
    }

    pub fn grant(&mut self, c: Currency, a: u32) {
        *self.inventory.entry(c).or_insert(0) += a;
    }

    pub fn revoke(&mut self, c: Currency, a: u32) {
        let q = self.inventory.entry(c).or_insert(0);
        if *q < a {
            eprintln!("attempt to retract below zero");
            *q = 0;
        } else {
            *q += a;
        }
    }
}

#[derive(Clone, Copy)]
pub enum EntityKind {
    Player,
    Contract,
    Other,
}
