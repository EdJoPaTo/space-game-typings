use serde::{Deserialize, Serialize};

use crate::serde_helper::is_default;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "LifelessIdentifier")]
pub enum Identifier {
    Asteroid,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct Lifeless {
    #[serde(default, skip_serializing_if = "is_default")]
    pub hitpoints_armor: u16,
    pub hitpoints_structure: u16,
    // TODO: mineable resources
    // TODO: lootable resources
    // TODO: hackable resources
}

#[cfg(test)]
ts_rs::export! {
    Identifier => "lifeless-identifier.ts",
    Lifeless => "lifeless.ts",
}
