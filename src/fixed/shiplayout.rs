use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct ShipLayout {
    pub slots_targeted: u8,
    pub slots_untargeted: u8,
    pub slots_passive: u8,

    pub cpu: u32,
    pub powergrid: u32,
    pub capacitor: u32,
    pub capacitor_recharge: u32,

    pub hitpoints_armor: u32,
    pub hitpoints_structure: u32,
}

#[cfg(test)]
ts_rs::export! {ShipLayout => "ship-layout.ts"}
