use serde::{Deserialize, Serialize};

use crate::fixed::shiplayout::ShipQualities;
use crate::serde_helper::ordered_map;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "ModulePassive")]
pub enum Passive {
    RookieArmorPlate,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "ModulePassiveDetails")]
pub struct Details {
    pub required_cpu: u16,
    pub required_powergrid: u16,

    #[serde(serialize_with = "ordered_map")]
    pub qualities: ShipQualities,
}

#[cfg(test)]
ts_rs::export! {
    Passive => "module-passive.ts",
    Details => "module-passive-details.ts",
}
