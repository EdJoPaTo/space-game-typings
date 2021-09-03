use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::serde_helper::ordered_map;

use super::Mineral;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "ItemCategory")]
pub enum Category {
    Mineral,
    Module,
    Ore,
    Ship,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "ItemDetails")]
pub struct Details {
    /// This is mainly added for TypeScript to get more easily what kind of item this is.
    /// As the Item Enum is untagged for serialization reasons the JSON / TypeScript typings only contain the item, not the general class.
    /// Also categories are a bit broader: multiple modules are just modules.
    pub category: Category,

    /// The amount of minerals returned when recycling one item
    #[serde(serialize_with = "ordered_map")]
    pub recycle: HashMap<Mineral, u32>,
    // TODO: volume for storage in ships
}
