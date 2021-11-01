use std::cmp::Ordering;
use std::collections::VecDeque;
use std::mem;

use chrono::naive::{NaiveDateTime, MIN_DATETIME};

use crate::{
    model::{
        dates, Amount, Currency, DatedHistoryEntry, Entities, Entity, EntityKind, FullUnit,
        HistoryEntry,
    },
    parser::{
        common::Operator,
        gsdl::Directive,
        tll::{AtomicTransaction, Command, Line, Transaction},
    },
};

pub struct Context {
    forbes: u32,
    notes: Vec<String>,
    assets: Vec<Currency>,
    entities: Entities,
    flotation: f32,
    total_buoyancy: u32,
    buoyancy_target: u32,
    datetime: NaiveDateTime,
    max_datetime: NaiveDateTime,
    history: VecDeque<DatedHistoryEntry>,
}

impl Context {
    pub fn from_datetime(dt: NaiveDateTime) -> Context {
        Context {
            forbes: 500,
            notes: Vec::new(),
            assets: Vec::new(),
            entities: Entities::new(),
            flotation: 1.0,
            total_buoyancy: 0,
            buoyancy_target: 0,
            datetime: MIN_DATETIME,
            max_datetime: dt,
            history: VecDeque::new(),
        }
    }

    pub fn relevel(&mut self, tb: u32) -> f32 {
        if self.datetime < dates::proposal_8557() {
            self.total_buoyancy = tb;
            self.flotation = tb as f32 / 2500.0;
        } else {
            self.total_buoyancy = self.buoyancy_target;
            self.flotation = (self.total_buoyancy as f32 / 2500.0).ceil();
        }
        self.flotation
    }

    pub fn nuke(&mut self) {
        let mut deletions = VecDeque::new();
        let mut grants = VecDeque::new();
        let assets = self.assets.clone();
        for ent in self.entities.as_sorted_vec().iter() {
            let name = ent.identifier();
            let should_grant_cards = if let EntityKind::Player(pp) = ent.kind() {
                pp.activity.is_active()
            } else {
                false
            };

            for currency in assets.iter() {
                if *currency != Currency::Coin {
                    deletions.push_back(Transaction::new(
                        name.clone(),
                        Amount::AllOf(*currency),
                        Operator::Minus,
                        "ECONOMY NUKE!".to_string(),
                    ));
                }

                if currency.is_card() && should_grant_cards {
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
            self.apply(self.datetime, &trans);
        }
    }

    pub fn payday(&mut self) {
        let mut transactions = VecDeque::new();
        for ent in self.entities.as_sorted_vec().iter() {
            match ent.kind() {
                EntityKind::Player(pp) => {
                    if pp.activity.is_active() {
                        transactions.push_back(Transaction::new(
                            ent.identifier().clone(),
                            Amount::PartOf(FullUnit::Boatload(Currency::Coin), 10),
                            Operator::Plus,
                            "Payday".to_string(),
                        ));
                    }
                }
                EntityKind::Contract(cp) => {
                    if cp.donation_level > 0 {
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
                            Amount::PartOf(FullUnit::Boatload(Currency::Coin), cp.donation_level),
                            Operator::Plus,
                            format!("Payday: donation level={}", cp.donation_level),
                        ));
                    }
                }
                _ => (),
            }
        }

        for trans in transactions.iter() {
            self.apply(self.datetime, &trans);
        }
    }

    pub fn enter(&mut self, line: Line) -> bool {
        if self.verify_datetime(line.datetime()) {
            self.datetime = line.datetime();
        } else {
            panic!(
                "traveling back in time! {} is before {}.",
                line.datetime().format("%F %R"),
                self.datetime.format("%F %R")
            );
        }

        if !self.verify_max_datetime(line.datetime()) {
            return false;
        }

        self.exec(line.datetime(), &line.action(), true);

        true
    }

    fn apply(&mut self, dt: NaiveDateTime, trans: &Transaction) {
        for t in self.expand_transaction(trans) {
            let player = self.entity_mut(t.agent());

            match t.amount().cmp(&0) {
                Ordering::Greater => player.grant(t.currency(), t.amount() as u32),
                Ordering::Less => player.revoke(t.currency(), t.amount().abs() as u32),
                Ordering::Equal => (),
            }

            if t.amount() != 0 {
                self.history.push_back(DatedHistoryEntry::new(
                    dt,
                    HistoryEntry::Transaction(t.clone()),
                ))
            }
        }
    }

    fn exec(&mut self, dt: NaiveDateTime, com: &Command, record: bool) {
        let option = match com {
            Command::Activate(name) => {
                self.entity_mut(name).activate();
                Some(format!("  {} becomes active", name))
            }
            Command::BuoyancyTarget(bt) => {
                self.buoyancy_target = *bt;
                Some(format!("  Buoyancy target set to {}", bt))
            }
            Command::Deactivate(name) => {
                self.entity_mut(name).deactivate();
                Some(format!("  {} becomes inactive", name))
            }
            Command::Deregister(identifier) => {
                self.deregister(&identifier);
                None
            }
            Command::Message(s) => {
                Some(format!("  {}", s))
            }
            Command::NewContract(identifier, full_name) => {
                self.entities
                    .insert(Entity::contract(identifier.clone(), full_name.clone()));
                Some(format!("  Contract {} created", identifier))
            }
            Command::NewPlayer(identifier, full_name) => {
                self.entities
                    .add_player(identifier.clone(), full_name.clone());
                None
            }
            Command::NoRecord(c) => {
                self.exec(dt, c, false);
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
                let tb = opttb.unwrap_or(self.entities.currency_total(Currency::Coin));
                let uf = self.relevel(tb);

                Some(format!(
                    "  RELEVELING: TB={}, UF={:.4}",
                    self.total_buoyancy, uf
                ))
            }
            Command::Rename(first, second) => {
                for a in self.assets.iter_mut() {
                    if a == first {
                        *a = *second;
                        break;
                    }
                }
                for e in self.entities.as_vec_mut() {
                    e.rename(*first,*second);
                }
                None
            }
            Command::Report => {
                self.forbes -= 1;
                Some(String::from("  WEEKLY REPORT"))
            }
            Command::Revision => {
                self.forbes -= 1;
                Some(String::from("  REPORT REVISION"))
            }
            Command::Transaction(t) => {
                self.apply(dt, &t);
                None
            }
        };

        if let Some(message) = option {
            if record {
                self.history
                    .push_back(DatedHistoryEntry::new(dt, HistoryEntry::Event(message)));
            }
        }
    }

    pub fn process(&mut self, dir: &Directive) {
        match dir {
            Directive::Assets(v) => {
                self.assets = v.clone();
            }
            Directive::Date(d) => {
                self.datetime = d.and_hms(0,0,0);
            }
            Directive::Entity(e) => {
                self.entities.insert(e.clone());
            }
            Directive::Flotation(f) => {
                self.flotation = *f;
            }
            Directive::Forbes(i) => {
                self.forbes = *i;
            }
        }
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
            self.apply(self.datetime, &trans);
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

    pub fn to_boatloads(&self, amt: u32) -> f32 {
        amt as f32 / self.flotation
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
                    Operator::Transfer(ref patient) => {
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
                    Operator::Transfer(ref patient) => AtomicTransaction::transfer_vec(
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
                    Operator::Transfer(ref patient) => AtomicTransaction::transfer_vec(
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

    pub fn verify_max_datetime(&self, other: NaiveDateTime) -> bool {
        other <= self.max_datetime
    }

    pub fn take_notes(&mut self) -> Vec<String> {
        mem::take(&mut self.notes)
    }

    pub fn assets(&self) -> &Vec<Currency> {
        &self.assets
    }

    pub fn flotation(&self) -> f32 {
        self.flotation
    }

    pub fn buoyancy_target(&self) -> u32 {
        self.buoyancy_target
    }

    pub fn total_buoyancy(&self) -> u32 {
        self.total_buoyancy
    }

    pub fn max_datetime(&self) -> NaiveDateTime {
        self.max_datetime
    }

    pub fn datetime(&self) -> NaiveDateTime {
        self.datetime
    }

    pub fn entities(&self) -> &Entities {
        &self.entities
    }

    pub fn forbes(&self) -> u32 {
        self.forbes
    }

    pub fn history(&self) -> &VecDeque<DatedHistoryEntry> {
        &self.history
    }
}
