use serde::{Deserialize, Serialize};

use crate::fixed::{module, shiplayout};

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "ShipFitting")]
pub struct Fitting {
    pub layout: shiplayout::Identifier,

    pub slots_targeted: Vec<module::TargetedIdentifier>,
    pub slots_untargeted: Vec<module::UntargetedIdentifier>,
    pub slots_passive: Vec<module::PassiveIdentifier>,
}

/// The current situation of the ship.
/// For the totals check the `ShipFitting`.
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "ShipStatus")]
pub struct Status {
    pub capacitor: u32,
    pub hitpoints_armor: u32,
    pub hitpoints_structure: u32,
}

#[cfg(test)]
ts_rs::export! {
    Fitting => "ship-fitting.ts",
    Status => "ship-status.ts",
}
