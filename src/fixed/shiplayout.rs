use serde::{Deserialize, Serialize};

pub type Identifier = String;

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct ShipLayout {
    pub slots_targeted: u8,
    pub slots_untargeted: u8,
    pub slots_passive: u8,

    pub cpu: u16,
    pub powergrid: u16,
    pub capacitor: u16,
    pub capacitor_recharge: u16,

    pub hitpoints_armor: u16,
    pub hitpoints_structure: u16,
    //
    // TODO: module effects like passives?
    // they could do stuff like recharge and are a kinda 'free' bonus system
}

#[cfg(test)]
ts_rs::export! {ShipLayout => "ship-layout.ts"}
