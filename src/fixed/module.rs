use serde::{Deserialize, Serialize};

use crate::serde_helper::ordered_vec;

pub type TargetedIdentifier = String;
pub type PassiveIdentifier = String;
pub type UntargetedIdentifier = String;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(
    rename_all = "camelCase",
    rename = "ModuleEffect",
    tag = "type",
    content = "amount"
)]
/// Effects in the order they get applied
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

    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacitor: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hitpoints_armor: Option<u16>,
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
