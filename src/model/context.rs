use std::collections::{HashMap, VecDeque};

use chrono::naive::{NaiveDateTime, MIN_DATE};
use tabular::{Row, Table};

use crate::{
    model::{DatedHistoryEntry, Entity, EntityKind, HistoryEntry},
    parser::{
        ast::{Amount, Currency, FullUnit, Operator},
        gsdl::Directive,
        tll::{AtomicTransaction, Command, Line, Statement, Transaction},
    },
};

pub struct Context {
    assets: Vec<Currency>,
    entities: HashMap<String, Entity>,
    flotation: f32,
    datetime: NaiveDateTime,
    history: VecDeque<DatedHistoryEntry>,
}

impl Context {
    pub fn new() -> Context {
        Context {
            assets: Vec::new(),
            entities: HashMap::new(),
            flotation: 1.0,
            datetime: MIN_DATE.and_hms(0, 0, 0),
            history: VecDeque::new(),
        }
    }

    pub fn relevel(&mut self) -> (u32, f32) {
        let tb = self
            .entities
            .values()
            .map(|ent| ent.balance(Currency::Coin))
            .sum::<u32>();
        let uf = tb as f32 / 2500.0;

        self.flotation = uf;
        (tb, uf)
    }

    pub fn nuke(&mut self) {
        let mut deletions = VecDeque::new();
        let mut grants = VecDeque::new();
        let assets = self.assets.clone();
        for (name, ent) in self.entities.iter() {
            for currency in assets.iter() {
                if *currency != Currency::Coin {
                    deletions.push_back(Transaction::new(
                        name.clone(),
                        Amount::AllOf(*currency),
                        Operator::Minus,
                        "ECONOMY NUKE!".to_string(),
                    ));
                }

                if currency.is_card() && ent.kind() == EntityKind::Player {
                    grants.push_back(Transaction::new(
                        name.clone(),
                        Amount::PartOf(FullUnit::Bare(*currency), 1),
                        Operator::Plus,
                        "ECONOMY NUKE!".to_string(),
                    ))
                }
            }
        }

        deletions.append(&mut grants);

        for trans in deletions.iter() {
            self.apply(&trans).iter().for_each(|e| {
                self.history
                    .push_back(DatedHistoryEntry::new(self.datetime, e.clone()))
            });
        }
    }

    pub fn enter(&mut self, line: Line) {
        if self.verify_datetime(line.datetime()) {
            self.datetime = line.datetime();
        } else {
            panic!(
                "traveling back in time! {} is before {}.",
                line.datetime().format("%F %R"),
                self.datetime.format("%F %R")
            );
        }

        match line.action() {
            Statement::Transaction(t) => self.apply(&t).iter().for_each(|e| {
                self.history
                    .push_back(DatedHistoryEntry::new(self.datetime, e.clone()))
            }),
            Statement::Command(c) => {
                let e = self.exec(&c);
                if e.is_some() {
                    self.history
                        .push_back(DatedHistoryEntry::new(self.datetime, e.unwrap().clone()))
                }
            }
        };
    }

    fn apply(&mut self, trans: &Transaction) -> Vec<HistoryEntry> {
        let mut result = Vec::new();
        for t in self.expand_transaction(trans) {
            let player = self.entity_mut(t.agent());

            if t.amount() > 0 {
                player.grant(t.currency(), t.amount() as u32);
            } else if t.amount() < 0 {
                player.revoke(t.currency(), t.amount().abs() as u32);
            }

            if t.amount() != 0 {
                result.push(HistoryEntry::Transaction(t.clone()))
            }
        }
        result
    }

    fn exec(&mut self, com: &Command) -> Option<HistoryEntry> {
        match com {
            Command::Relevel => {
                let (tb, uf) = self.relevel();
                Some(format!("-- RELEVELING: TB={}, UF={:.4}", tb, uf))
            }
            Command::NewPlayer(identifier, full_name) => {
                self.add_player(identifier.clone(), full_name.clone());
                None
            }
            Command::Nuke => {
                self.nuke();
                None
            }
        }
        .map(|s| HistoryEntry::Event(s))
    }

    pub fn process(&mut self, dir: &Directive) {
        match dir {
            Directive::Assets(v) => {
                self.assets = v.clone();
            }
            Directive::Flotation(f) => {
                self.flotation = f.clone();
            }
            Directive::Entity(e) => {
                self.insert_entity(e.clone());
            }
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

    pub fn entity(&self, identifier: &str) -> &Entity {
        self.entities
            .get(identifier)
            .expect(&format!("no such entity: {}", identifier))
    }

    pub fn entity_mut(&mut self, identifier: &str) -> &mut Entity {
        self.entities
            .get_mut(identifier)
            .expect(&format!("no such entity: {}", identifier))
    }

    pub fn boatloads(&self, amt: f32) -> u32 {
        (self.flotation * amt).ceil() as u32
    }

    pub fn expand_transaction(&self, trans: &Transaction) -> Vec<AtomicTransaction> {
        match trans.amount() {
            Amount::Everything => {
                let plus = self
                    .assets
                    .iter()
                    .map(|c| (self.entity(trans.agent()).balance(*c) as i32, c));

                match trans.operator() {
                    Operator::Minus => plus
                        .map(|x| {
                            AtomicTransaction::new(
                                trans.agent().to_string(),
                                x.0,
                                *x.1,
                                trans.comment().to_string(),
                            )
                        })
                        .collect::<Vec<AtomicTransaction>>(),
                    Operator::Transfer(patient) => {
                        let mut result = Vec::new();
                        for t in plus {
                            result.append(&mut AtomicTransaction::transfer_vec(
                                trans.agent(),
                                &patient,
                                t.0,
                                *t.1,
                                trans.comment(),
                            ));
                        }
                        result
                    }
                    Operator::Plus => panic!("cannot add everything"),
                }
            }
            Amount::AllOf(c) => {
                let balance = self.entity(trans.agent()).balance(c) as i32;
                match trans.operator() {
                    Operator::Minus => vec![AtomicTransaction::new(
                        trans.agent().to_string(),
                        -balance,
                        c,
                        trans.comment().to_string(),
                    )],
                    Operator::Transfer(patient) => AtomicTransaction::transfer_vec(
                        trans.agent(),
                        &patient,
                        balance,
                        c,
                        trans.comment(),
                    ),
                    Operator::Plus => panic!("cannot add all of something"),
                }
            }
            Amount::PartOf(unit, amt) => {
                let (amount, currency) = match unit {
                    FullUnit::Bare(c) => (amt as i32, c),
                    FullUnit::Boatload(c) => (self.boatloads(amt as f32) as i32, c),
                };

                match trans.operator() {
                    Operator::Minus => vec![AtomicTransaction::new(
                        trans.agent().to_string(),
                        -amount,
                        currency,
                        trans.comment().to_string(),
                    )],
                    Operator::Transfer(patient) => AtomicTransaction::transfer_vec(
                        trans.agent(),
                        &patient,
                        amount,
                        currency,
                        trans.comment(),
                    ),
                    Operator::Plus => vec![AtomicTransaction::new(
                        trans.agent().to_string(),
                        amount,
                        currency,
                        trans.comment().to_string(),
                    )],
                }
            }
        }
    }

    pub fn verify_datetime(&self, other: NaiveDateTime) -> bool {
        other >= self.datetime
    }

    pub fn report(&self) -> String {
        let tstr = "[{:<}] {:<} {:<}{:>}{:<} {:<}";
        let table = &mut Table::new(&format!(
            "{}{}",
            " {:<}",
            "  {:>}".repeat(self.assets.len())
        ));
        let mut date = MIN_DATE;
        let mut result = String::from("");

        {
            let mut row = Row::new().with_cell("=".repeat(13));

            self.assets.iter().for_each(|_| {
                row.add_cell("====");
            });

            table.add_row(row);
        }

        for (name, ent) in self.entities.iter() {
            let mut row = Row::new().with_cell(name.replace("_", " "));

            for curr in self.assets.iter() {
                row.add_cell(ent.balance(*curr));
            }

            table.add_row(row);
        }

        for entry in self.history.iter().rev() {
            if entry.datetime().date() != date {
                result = format!("{}{}", result, *table);
                *table = Table::new(tstr)
                    .with_heading("")
                    .with_heading(entry.datetime().format("*** %a %-d %B %Y").to_string());

                date = entry.datetime().date();
            }

            match entry.entry() {
                HistoryEntry::Transaction(t) => table.add_row(
                    Row::new()
                        .with_cell(entry.datetime().format("%R"))
                        .with_cell(format!("{}:", t.agent().replace("_", " ")))
                        .with_cell(if t.amount() > 0 { "+" } else { "-" })
                        .with_cell(t.amount().abs())
                        .with_cell(t.currency().abbr())
                        .with_cell(format!("({})", t.comment())),
                ),
                HistoryEntry::Event(s) => {
                    table.add_heading(format!("[{}] {}", entry.datetime().format("%R"), s))
                }
            };
        }
        format!("{}{}", result, *table)
    }
}
