use crate::parser::tll::AtomicTransaction;

use chrono::naive::NaiveDateTime;

#[derive(Clone)]
pub enum HistoryEntry {
    Transaction(AtomicTransaction),
    Event(String),
}

#[derive(Clone)]
pub struct DatedHistoryEntry {
    datetime: NaiveDateTime,
    entry: HistoryEntry,
}

impl DatedHistoryEntry {
    pub fn new(datetime: NaiveDateTime, entry: HistoryEntry) -> DatedHistoryEntry {
        DatedHistoryEntry { datetime, entry }
    }

    pub fn datetime(&self) -> NaiveDateTime {
        self.datetime
    }

    pub fn entry(&self) -> &HistoryEntry {
        &self.entry
    }
}
