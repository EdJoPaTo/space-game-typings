use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::serde_helper::ordered_map;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "ShipLayoutIdentifier")]
pub enum Identifier {
    RookieShip,
    Frigate,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub enum ShipQuality {
    HitpointsArmor,
    HitpointsStructure,

    Capacitor,
    CapacitorRecharge,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct ShipLayout {
    pub slots_targeted: u8,
    pub slots_untargeted: u8,
    pub slots_passive: u8,

    pub cpu: u16,
    pub powergrid: u16,

    #[serde(default, serialize_with = "ordered_map")]
    pub qualities: HashMap<ShipQuality, i16>,
}

#[cfg(test)]
ts_rs::export! {
    Identifier => "ship-layout-identifier.ts",
    ShipQuality => "ship-quality.ts",
    ShipLayout => "ship-layout.ts",
}
