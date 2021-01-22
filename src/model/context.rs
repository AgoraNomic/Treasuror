use std::collections::HashMap;

use chrono::naive::{NaiveDateTime, MIN_DATE};

use crate::{
    match_first_pop,
    model::Entity,
    parser::{
        ast::{Amount, Currency, FullUnit, Operator, Token},
        gsdl::Directive,
        tll::{Command, Line},
    },
};

pub struct Context {
    assets: Vec<Currency>,
    entities: HashMap<String, Entity>,
    flotation: f32,
    datetime: NaiveDateTime,
}

impl Context {
    pub fn new() -> Context {
        Context {
            assets: Vec::new(),
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

    pub fn process(&mut self, dir: &Directive) {
        match dir {
            Directive::Assets(v) => { self.assets = v.clone(); }
            Directive::Flotation(f) => { self.flotation = f.clone(); }
            Directive::Entity(e) => { self.insert_entity(e.clone()); }
        }
    }
    
    pub fn insert_entity(&mut self, ent: Entity) {
        if self.entities.get(&ent.identifier()[..]).is_some() {
            panic!("entity {} already exists", ent.identifier());
        }
        self.entities.insert(ent.identifier().clone(), ent);
    }

    pub fn add_player(&mut self, identifier: String, full_name: String) {
        self.insert_entity(Entity::player(identifier, full_name));
    }

    pub fn display(&self) -> String {
        let mut result = String::new();
        for (name, ent) in self.entities.iter() {
            result.push_str(name);
            result.push_str(": ");
            for curr in self.assets.iter() {
                result.push_str(&ent.inventory().get(&curr).unwrap_or(&0).to_string());
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
