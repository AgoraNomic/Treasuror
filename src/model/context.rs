use std::cmp::Ordering;
use std::collections::{HashMap, VecDeque};
use std::mem;

use chrono::naive::{NaiveDateTime, MIN_DATETIME};

use crate::{
    model::{dates, DatedHistoryEntry, Entity, EntityKind, HistoryEntry},
    parser::{
        ast::{Amount, Currency, FullUnit, Operator},
        gsdl::Directive,
        tll::{AtomicTransaction, Command, Line, Statement, Transaction},
    },
};

pub struct Context {
    forbes: u32,
    notes: Vec<String>,
    assets: Vec<Currency>,
    entities: HashMap<String, Entity>,
    flotation: f32,
    total_buoyancy: u32,
    buoyancy_target: u32,
    datetime: NaiveDateTime,
    history: VecDeque<DatedHistoryEntry>,
}

impl Context {
    pub fn new() -> Context {
        Context {
            forbes: 500,
            notes: Vec::new(),
            assets: Vec::new(),
            entities: HashMap::new(),
            flotation: 1.0,
            total_buoyancy: 0,
            buoyancy_target: 0,
            datetime: MIN_DATETIME,
            history: VecDeque::new(),
        }
    }

    pub fn relevel(&mut self, tb: u32) -> f32 {
        if self.datetime < dates::proposal_8557() {
            self.total_buoyancy = tb;
            self.flotation = tb as f32 / 2500.0;
        } else {
            self.total_buoyancy = self.buoyancy_target;
            self.flotation = (tb as f32 / 2500.0).ceil();
        }
        self.flotation
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

                if currency.is_card() && ent.kind() == EntityKind::Player(true) {
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

    pub fn payday(&mut self) {
        let mut transactions = VecDeque::new();
        for ent in self.entities_vec_sorted().iter() {
            match ent.kind() {
                EntityKind::Player(true) => {
                    transactions.push_back(Transaction::new(
                        ent.identifier().clone(),
                        Amount::PartOf(FullUnit::Boatload(Currency::Coin), 10),
                        Operator::Plus,
                        "Payday".to_string(),
                    ));
                }
                EntityKind::Contract => {
                    if ent.donation_level() > 0 {
                        transactions.push_back(Transaction::new(
                            ent.identifier().clone(),
                            Amount::PartOf(
                                FullUnit::Bare(Currency::Coin),
                                (ent.balance(Currency::Coin) as f32 / 2.0).floor() as u32,
                            ),
                            Operator::Minus,
                            "Payday: charity coin destruction".to_string(),
                        ));
                        transactions.push_back(Transaction::new(
                            ent.identifier().clone(),
                            Amount::PartOf(
                                FullUnit::Boatload(Currency::Coin),
                                ent.donation_level(),
                            ),
                            Operator::Plus,
                            format!("Payday: donation level={}", ent.donation_level()),
                        ));
                    }
                }
                _ => (),
            }
        }

        for trans in transactions.iter() {
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
                if let Some(message) = self.exec(&c) {
                    self.history
                        .push_back(DatedHistoryEntry::new(self.datetime, message))
                }
            }
        };
    }

    fn apply(&mut self, trans: &Transaction) -> Vec<HistoryEntry> {
        let mut result = Vec::new();
        for t in self.expand_transaction(trans) {
            let player = self.entity_mut(t.agent());

            match t.amount().cmp(&0) {
                Ordering::Greater => player.grant(t.currency(), t.amount() as u32),
                Ordering::Less => player.revoke(t.currency(), t.amount().abs() as u32),
                Ordering::Equal => (),
            }

            if t.amount() != 0 {
                result.push(HistoryEntry::Transaction(t.clone()))
            }
        }
        result
    }

    fn exec(&mut self, com: &Command) -> Option<HistoryEntry> {
        match com {
            Command::Activate(name) => {
                self.entity_mut(name).activate();
                Some(format!("{} becomes active", name))
            }
            Command::BuoyancyTarget(bt) => {
                self.buoyancy_target = *bt;
                Some(format!("Buoyancy target set to {}", bt))
            }
            Command::Deactivate(name) => {
                self.entity_mut(name).deactivate();
                Some(format!("{} becomes inactive", name))
            }
            Command::Deregister(identifier) => {
                self.deregister(&identifier);
                None
            }
            Command::NewContract(identifier, full_name) => {
                self.insert_entity(Entity::contract(identifier.clone(), full_name.clone()));
                Some(format!("Contract {} created", identifier))
            }
            Command::NewPlayer(identifier, full_name) => {
                self.add_player(identifier.clone(), full_name.clone());
                None
            }
            Command::Nuke => {
                self.nuke();
                None
            }
            Command::Payday => {
                self.payday();
                None
            }
            Command::Relevel(opttb) => {
                let tb = opttb.unwrap_or(self.currency_total(Currency::Coin));
                let uf = self.relevel(tb);

                Some(format!("  RELEVELING: TB={}, UF={:.4}", tb, uf))
            }
            Command::Report => {
                self.forbes -= 1;
                Some(String::from("  WEEKLY REPORT"))
            }
            Command::Revision => {
                self.forbes -= 1;
                Some(String::from("  REPORT REVISION"))
            }
        }
        .map(HistoryEntry::Event)
    }

    pub fn process(&mut self, dir: &Directive) {
        match dir {
            Directive::Assets(v) => {
                self.assets = v.clone();
            }
            Directive::Entity(e) => {
                self.insert_entity(e.clone());
            }
            Directive::Flotation(f) => {
                self.flotation = *f;
            }
            Directive::Forbes(i) => {
                self.forbes = *i;
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

    pub fn deregister(&mut self, identifier: &str) {
        let transactions = {
            let entity = self.entity(&identifier);

            let comment = String::from("Indeterminate owner: was ") + &entity.identifier();

            self.assets
                .iter()
                .map(|c| {
                    Transaction::new(
                        "L&F_Dept.".to_string(),
                        Amount::PartOf(FullUnit::Bare(*c), entity.balance(*c)),
                        Operator::Plus,
                        comment.clone(),
                    )
                })
                .collect::<Vec<Transaction>>()
        };

        for trans in transactions {
            self.apply(&trans).iter().for_each(|e| {
                self.history
                    .push_back(DatedHistoryEntry::new(self.datetime, e.clone()))
            });
        }

        self.entities.remove(identifier);
    }

    pub fn entity(&self, identifier: &str) -> &Entity {
        self.entities
            .get(identifier)
            .unwrap_or_else(|| panic!("no such entity: {}", identifier))
    }

    pub fn entity_mut(&mut self, identifier: &str) -> &mut Entity {
        self.entities
            .get_mut(identifier)
            .unwrap_or_else(|| panic!("no such entity: {}", identifier))
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

    pub fn take_notes(&mut self) -> Vec<String> {
        mem::take(&mut self.notes)
    }

    pub fn entities_vec_sorted(&self) -> Vec<&Entity> {
        let mut entities = self.entities.values().collect::<Vec<&Entity>>();
        entities.sort_by(|a, b| {
            a.identifier()
                .to_lowercase()
                .cmp(&b.identifier().to_lowercase())
        });
        entities
    }

    pub fn assets(&self) -> &Vec<Currency> {
        &self.assets
    }

    pub fn buoyancy_target(&self) -> u32 {
        self.buoyancy_target
    }

    pub fn total_buoyancy(&self) -> u32 {
        self.total_buoyancy
    }

    pub fn currency_total(&self, curr: Currency) -> u32 {
        self.entities
            .values()
            .map(|ent| ent.balance(curr))
            .sum::<u32>()
    }

    pub fn datetime(&self) -> NaiveDateTime {
        self.datetime
    }

    pub fn entities(&self) -> &HashMap<String, Entity> {
        &self.entities
    }

    pub fn forbes(&self) -> u32 {
        self.forbes
    }

    pub fn history(&self) -> &VecDeque<DatedHistoryEntry> {
        &self.history
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}
