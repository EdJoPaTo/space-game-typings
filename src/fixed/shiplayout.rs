use serde::{Deserialize, Serialize};

use crate::entity::Collateral;
use crate::serde_helper::ordered_vec;

use super::round_effect::RoundEffect;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub enum ShipClass {
    RookieShip,
    Frigate,
    Cruiser,
    Battleship,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub enum ShipLayout {
    /// Special Ship used by the Guardians to secure the High Sec
    Paladin,

    /// Beginner default ship
    Abis,

    Hecate,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "ShipLayoutDetails")]
pub struct Details {
    pub class: ShipClass,

    pub cpu: u16,
    pub powergrid: u16,

    pub slots_targeted: u8,
    pub slots_untargeted: u8,
    pub slots_passive: u8,

    /// Total slots that can be used for stored goods.
    /// Will probably be replaced in the future with a `cargo_volume` or something like that.
    pub cargo_slots: u32,

    #[serde(flatten)]
    pub collateral: Collateral,

    #[serde(default, serialize_with = "ordered_vec")]
    pub round_effects: Vec<RoundEffect>,
}

#[cfg(feature = "typescript")]
ts_rs::export! {
    ShipClass,
    Details,
    ShipLayout => "typescript/generated-ship-layout.ts"
}

impl std::fmt::Display for ShipLayout {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Default for ShipLayout {
    fn default() -> Self {
        Self::Abis
    }
}
