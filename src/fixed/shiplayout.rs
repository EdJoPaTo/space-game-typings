use serde::{Deserialize, Serialize};

use crate::persist::ship::Status;
use crate::serde_helper::ordered_vec;

use super::round_effect::RoundEffect;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub enum ShipClass {
    RookieShip,
    Frigate,
    Cruiser,
    Battleship,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(test, derive(ts_rs::TS))]
pub enum ShipLayout {
    /// Special Ship used by the Guardians to secure the High Sec
    Paladin,

    /// Beginner default ship
    Abis,

    Hecate,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "ShipLayoutDetails")]
pub struct Details {
    pub class: ShipClass,

    pub slots_targeted: u8,
    pub slots_untargeted: u8,
    pub slots_passive: u8,

    pub cpu: u16,
    pub powergrid: u16,

    #[serde(flatten)]
    pub status: Status,

    #[serde(default, serialize_with = "ordered_vec")]
    pub round_effects: Vec<RoundEffect>,
}

#[cfg(test)]
ts_rs::export! {
    ShipClass => "ship-class.ts",
    ShipLayout => "ship-layout.ts",
    Details => "ship-layout-details.ts",
}

impl std::fmt::Display for ShipLayout {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
