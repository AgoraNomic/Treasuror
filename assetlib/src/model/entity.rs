use std::mem;
use std::collections::hash_map::{HashMap, Entry};
use std::fmt::{self, Display};

use crate::{
    match_first_pop,
    model::{Currency, Inventory},
    parser::common::Token,
};

pub struct Entities {
    contents: HashMap<String, Entity>,
}

impl Entities {
    pub fn new() -> Entities {
        Entities {
            contents: HashMap::new(),
        }
    }

    pub fn add_player(&mut self, identifier: String, full_name: String) {
        self.insert(Entity::player(identifier, full_name));
    }

    pub fn remove(&mut self, identifier: &str) -> Option<Entity> {
        self.contents.remove(identifier)
    }

    pub fn get(&self, identifier: &str) -> Option<&Entity> {
        self.contents.get(identifier)
    }

    pub fn get_mut(&mut self, identifier: &str) -> Option<&mut Entity> {
        self.contents.get_mut(identifier)
    }

    pub fn insert(&mut self, ent: Entity) {
        if let Entry::Vacant(o) = self.contents.entry(ent.identifier().clone()) {
            o.insert(ent);
        } else {
            panic!("entity {} already exists", ent.identifier());
        }
    }

    pub fn as_sorted_vec(&self) -> Vec<&Entity> {
        let mut entities = self.contents.values().collect::<Vec<&Entity>>();
        entities.sort_by(|a, b| {
            a.identifier()
                .to_lowercase()
                .cmp(&b.identifier().to_lowercase())
        });
        entities
    }

    pub fn as_grouped_vec(&self) -> Vec<(EntityKind, Vec<&Entity>)> {
        let mut entities = self.as_sorted_vec();
        entities.sort_by_key(|e| e.kind());

        let mut result = Vec::new();
        let mut curkind = None;
        let subvec = &mut Vec::new();
        for e in entities.iter() {
            if let Some(k) = curkind {
                if e.kind() != k {
                    result.push((k, mem::take(subvec)));
                    curkind = Some(e.kind());
                }
            } else {
                curkind = Some(e.kind());
            }
            subvec.push(*e)
        }
        if let Some(k) = curkind {
            result.push((k, mem::take(subvec)));
        }
        
        result
    }

    pub fn currency_total(&self, curr: Currency) -> u32 {
        self.contents
            .values()
            .map(|ent| ent.balance(curr))
            .sum::<u32>()
    }
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct Entity {
    full_name: String,
    identifier: String,
    kind: EntityKind,
    inventory: Inventory,
}

impl Entity {
    pub fn from_vec(tokens: &mut Vec<Token>) -> Entity {
        let mut kind = match_first_pop!(tokens {
            Token::Identifier(s) => { match &s.to_lowercase()[..] {
                "p" => EntityKind::Player(PlayerParams::new()),
                "c" => EntityKind::Contract(ContractParams::new()),
                "o" => EntityKind::Other,
                _ => panic!("Expected 'P', 'C', or 'O'"),
            }},
        } else { panic!("Expected first arg of ENT directive to be identifier") });

        if let EntityKind::Contract(ref mut cp) = kind {
            cp.donation_level = match_first_pop!(tokens {
                Token::Integer(i) => { i },
            } else { 0 })
        }

        let identifier = match_first_pop!(tokens {
            Token::Identifier(s) => { s },
        } else { panic!("Expected name") });

        let full_name = match_first_pop!(tokens {
            Token::String(s) => { s },
        } else { identifier.clone() });

        let mut inventory: Inventory = HashMap::new();
        while !tokens.is_empty() {
            let amount = match_first_pop!(tokens {
                Token::Integer(i) => { i },
            } else { panic!("expected number") });

            let currency = match_first_pop!(
                tokens {
                    Token::Identifier(s) => {
                        Currency::from_abbr(&s).unwrap_or_else(
                            || panic!("invalid currency: '{}'!", s)
                        )
                    },
                } else { panic!("expected currency identifier") }
            );

            inventory.insert(currency, amount);
        }

        Entity {
            full_name,
            identifier,
            kind,
            inventory,
        }
    }

    pub fn player(identifier: String, full_name: String) -> Entity {
        Entity {
            full_name,
            identifier,
            kind: EntityKind::Player(PlayerParams::new()),
            inventory: HashMap::with_capacity(5),
        }
    }

    pub fn contract(identifier: String, full_name: String) -> Entity {
        Entity {
            full_name,
            identifier,
            kind: EntityKind::Contract(ContractParams::new()),
            inventory: HashMap::new(),
        }
    }

    pub fn balance(&self, c: Currency) -> u32 {
        *self.inventory.get(&c).unwrap_or(&0)
    }

    pub fn grant(&mut self, c: Currency, a: u32) {
        *self.inventory.entry(c).or_insert(0) += a;
    }

    pub fn revoke(&mut self, c: Currency, a: u32) {
        let q = self.inventory.entry(c).or_insert(0);
        if *q < a {
            eprintln!(
                "attempt to retract from {} below zero: {} < {} ({})",
                self.full_name,
                *q,
                a,
                c.abbr()
            );
            *q = 0;
        } else {
            *q -= a;
        }
    }

    pub fn activate(&mut self) {
        if let EntityKind::Player(ref mut pp) = self.kind {
            pp.is_active = true;
        }
    }

    pub fn deactivate(&mut self) {
        if let EntityKind::Player(ref mut pp) = self.kind {
            pp.is_active = false;
        }
    }

    pub fn has_full_name(&self) -> bool {
        self.full_name != self.identifier
    }

    pub fn full_name(&self) -> &String {
        &self.full_name
    }

    pub fn identifier(&self) -> &String {
        &self.identifier
    }

    pub fn kind(&self) -> EntityKind {
        self.kind
    }

    pub fn inventory(&self) -> &Inventory {
        &self.inventory
    }
}

#[derive(Debug, Clone, Copy, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub enum EntityKind {
    Player(PlayerParams),
    Contract(ContractParams),
    Other,
}

impl Display for EntityKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad(&match self {
            EntityKind::Player(pp) => format!("Player({}a)", if pp.is_active { "+" } else { "-" }),
            EntityKind::Contract(cp) => format!("Contract({:02})", cp.donation_level),
            EntityKind::Other => String::from("Entity"),
        })
    }
}

#[derive(Debug, Clone, Copy, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct PlayerParams {
    pub is_active: bool,
}

impl PlayerParams {
    pub fn new() -> PlayerParams {
        PlayerParams { is_active: true }
    }
}

#[derive(Debug, Clone, Copy, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct ContractParams {
    pub donation_level: u32,
}

impl ContractParams {
    pub fn new() -> ContractParams {
        ContractParams { donation_level: 0 }
    }
}
