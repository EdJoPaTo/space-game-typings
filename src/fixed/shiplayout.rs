use serde::{Deserialize, Serialize};

use crate::persist::ship::Status;
use crate::serde_helper::ordered_vec;

use super::round_effect::RoundEffect;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub enum ShipLayout {
    /// Beginner default ship
    RookieShip,

    Frigate,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "ShipLayoutDetails")]
pub struct Details {
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
    ShipLayout => "ship-layout.ts",
    Details => "ship-layout-details.ts",
}
