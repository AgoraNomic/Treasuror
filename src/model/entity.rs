use std::collections::HashMap;

use crate::{
    match_first_pop,
    model::Inventory,
    parser::ast::{Currency, Token},
};

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
        let kind = match_first_pop!(tokens {
            Token::Identifier(s) => { match &s.to_lowercase()[..] {
                "p" => EntityKind::Player,
                "c"=> EntityKind::Contract,
                "o" => EntityKind::Other,
                _ => panic!("Expected 'P', 'C', or 'O'"),
            }},
        } else { panic!("Expected first arg of ENT directive to be identifier") });

        let identifier = match_first_pop!(tokens {
            Token::Identifier(s) => { s },
        } else { panic!("Expected name") });

        let full_name = match_first_pop!(tokens {
            Token::String(s) => { s },
        } else { identifier.clone() });

        let mut inventory: Inventory = HashMap::new();
        while tokens.len() > 0 {
            let amount = match_first_pop!(tokens {
                Token::Integer(i) => { i },
            } else { panic!("expected number") });

            let currency = match_first_pop!(
                tokens {
                    Token::Identifier(s) => {
                        Currency::from_str(&s).expect(
                            &format!("invalid currency: '{}'!", s)
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
            kind: EntityKind::Player,
            inventory: HashMap::with_capacity(5),
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
            eprintln!("attempt to retract below zero");
            *q = 0;
        } else {
            *q += a;
        }
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

#[derive(Clone, Copy)]
pub enum EntityKind {
    Player,
    Contract,
    Other,
}
