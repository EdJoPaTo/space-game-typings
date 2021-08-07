use serde::{Deserialize, Serialize};

pub type Identifier = String;

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct Lifeless {
    pub hitpoints_armor: u16,
    pub hitpoints_structure: u16,
    // TODO: mineable resources
    // TODO: lootable resources
    // TODO: hackable resources
}

#[cfg(test)]
ts_rs::export! {Lifeless => "lifeless.ts"}
