mod super::context;
mod super::entity;

pub type Inventory = HashMap<Currency, u32>;

pub use super::{
    context::Context,
    entity::{Entity, EntityKind},
}
