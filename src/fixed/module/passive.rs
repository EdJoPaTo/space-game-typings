use serde::{Deserialize, Serialize};

use crate::serde_helper::is_default;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS), ts(export))]
#[serde(rename_all = "camelCase", rename = "ModulePassive")]
pub enum Passive {
    RookieArmorPlate,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS), ts(export))]
#[serde(rename_all = "camelCase", rename = "ModulePassiveDetails")]
pub struct Details {
    pub required_cpu: u16,
    pub required_powergrid: u16,

    // Everything following will be added / removed from the ship that uses it
    #[serde(default, skip_serializing_if = "is_default")]
    pub hitpoints_armor: i16,
}
