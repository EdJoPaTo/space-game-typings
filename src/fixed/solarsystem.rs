use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::serde_helper::ordered_map;

pub type Identifier = String;

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct Solarsystem {
    pub name: String,
    /// Percentage
    pub security: u8,
    /// Amount
    pub planets: u8,

    /// Gates in the system.
    /// Key: Target System
    /// Value: The planet they are
    #[serde(serialize_with = "ordered_map")]
    pub stargates: HashMap<Identifier, u8>,

    /// Stations and at which planet they are.
    /// Example: [1,3] -> Station 1 is at Planet 1, Station 2 is at Planet 3
    pub stations: Vec<u8>,
}

#[cfg(test)]
ts_rs::export! {Solarsystem => "solarsystem.ts"}
