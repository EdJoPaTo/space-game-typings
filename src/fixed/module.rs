use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::serde_helper::{ordered_map, ordered_vec};

use super::shiplayout::ShipQuality;

pub type TargetedIdentifier = String;
pub type PassiveIdentifier = String;
pub type UntargetedIdentifier = String;

#[derive(Debug, Clone, Copy, Serialize, Hash, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(
    rename_all = "camelCase",
    rename = "ModuleEffect",
    tag = "type",
    content = "amount"
)]
pub enum Effect {
    Capacitor(i16),
    ArmorRepair(u16),
    Damage(u16),
    Mine(u16),
    WarpDisruption,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "ModulePassive")]
pub struct Passive {
    pub required_cpu: u16,
    pub required_powergrid: u16,

    #[serde(serialize_with = "ordered_map")]
    pub qualities: HashMap<ShipQuality, i16>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "ModuleUntargeted")]
pub struct Untargeted {
    pub required_cpu: u16,
    pub required_powergrid: u16,

    #[serde(serialize_with = "ordered_vec")]
    pub effects: Vec<Effect>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "ModuleTargeted")]
pub struct Targeted {
    pub required_cpu: u16,
    pub required_powergrid: u16,

    #[serde(serialize_with = "ordered_vec")]
    pub effects_origin: Vec<Effect>,
    #[serde(serialize_with = "ordered_vec")]
    pub effects_target: Vec<Effect>,
}

#[cfg(test)]
ts_rs::export! {
    Effect => "module-effect.ts",
    Passive => "module-passive.ts",
    Untargeted => "module-untargeted.ts",
    Targeted => "module-targeted.ts",
}
