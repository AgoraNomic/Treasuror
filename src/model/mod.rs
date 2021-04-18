use std::collections::HashMap;

mod amount;
mod context;
pub mod dates;
mod entity;
mod history;
mod operator;
mod report;
mod token;
mod unit;

pub type Inventory = HashMap<Currency, u32>;

pub use crate::model::{
    amount::Amount,
    context::Context,
    entity::{Entity, EntityKind},
    history::{DatedHistoryEntry, HistoryEntry},
    operator::Operator,
    report::Report,
    token::{Token, TokenIterator},
    unit::{Currency, FullUnit},
};
