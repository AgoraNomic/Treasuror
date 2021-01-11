use std::collections::HashMap;

use crate::parser::ast::Currency;

mod context;
mod entity;

pub type Inventory = HashMap<Currency, u32>;

pub use crate::model::{
    context::Context,
    entity::{Entity, EntityKind},
};
