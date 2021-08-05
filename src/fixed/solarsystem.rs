use std::collections::HashMap;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::serde_helper::ordered_map;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename = "SolarsystemIdentifier")]
pub enum Identifier {
    /// The home system for new players
    Wabinihwa,

    Arama,
    Iramil,
    Liagi,
    Plagar,
    Vosu,
}

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Default for Identifier {
    fn default() -> Self {
        Self::Wabinihwa
    }
}

impl FromStr for Identifier {
    type Err = serde_json::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(&format!(r#""{}""#, s))
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct Solarsystem {
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
ts_rs::export! {
    Identifier => "solarsystem-identifier.ts",
    Solarsystem => "solarsystem.ts",
}
