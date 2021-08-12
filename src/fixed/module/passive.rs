use serde::{Deserialize, Serialize};

use crate::serde_helper::is_default;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "ModulePassive")]
pub enum Passive {
    RookieArmorPlate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "ModulePassiveDetails")]
pub struct Details {
    pub required_cpu: u16,
    pub required_powergrid: u16,

    #[serde(default, skip_serializing_if = "is_default")]
    pub hitpoints_armor: i16,
}

#[cfg(test)]
ts_rs::export! {
    Passive => "module-passive.ts",
    Details => "module-passive-details.ts",
}
