use serde::{Deserialize, Serialize};

use crate::fixed::solarsystem::Solarsystem;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "PlayerGeneral")]
pub struct General {
    pub home_solarsystem: Solarsystem,
    pub home_station: u8,

    /// Paperclips are the currency
    pub paperclips: u64,
}

impl Default for General {
    fn default() -> Self {
        Self {
            home_solarsystem: Solarsystem::default(),
            home_station: 0,
            paperclips: 2000,
        }
    }
}
