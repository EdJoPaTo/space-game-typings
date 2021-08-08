use serde::{Deserialize, Serialize};

use crate::fixed::solarsystem;

pub type Identifier = String;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "PlayerGeneral")]
pub struct General {
    pub home_solarsystem: solarsystem::Identifier,
    pub home_station: u8,

    /// Paperclips are the currency
    pub paperclips: u64,
}

impl Default for General {
    fn default() -> Self {
        Self {
            home_solarsystem: solarsystem::Identifier::default(),
            home_station: 0,
            paperclips: 2000,
        }
    }
}

#[cfg(test)]
ts_rs::export! {
    General => "player-general.ts",
}

#[must_use]
pub fn parse_identifier(identifier: &str) -> Option<(String, String)> {
    let mut splitted = identifier.split('-');
    let prefix = splitted.next()?;
    if prefix == "player" {
        let platform = splitted.next()?;
        let unique = splitted.next()?;
        if splitted.next().is_none() {
            return Some((platform.to_string(), unique.to_string()));
        }
    }
    None
}
