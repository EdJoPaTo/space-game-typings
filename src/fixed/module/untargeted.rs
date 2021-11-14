use serde::{Deserialize, Serialize};

use crate::fixed::round_effect::RoundEffect;
use crate::serde_helper::ordered_vec;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS), ts(export))]
#[serde(rename_all = "camelCase", rename = "ModuleUntargeted")]
pub enum Untargeted {
    RookieArmorRepair,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS), ts(export))]
#[serde(rename_all = "camelCase", rename = "ModuleUntargetedDetails")]
pub struct Details {
    pub required_cpu: u16,
    pub required_powergrid: u16,

    #[serde(serialize_with = "ordered_vec")]
    pub effects: Vec<RoundEffect>,
}
