use std::collections::HashMap;

use crate::parser::ast::Currency;

mod context;
pub mod dates;
mod entity;
mod history;
mod report;

pub type Inventory = HashMap<Currency, u32>;

pub use crate::model::{
    context::Context,
    entity::{Entity, EntityKind},
    history::{DatedHistoryEntry, HistoryEntry},
    report::Report,
};
