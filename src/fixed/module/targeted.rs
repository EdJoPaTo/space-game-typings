use serde::{Deserialize, Serialize};

use crate::fixed::round_effect::RoundEffect;
use crate::serde_helper::ordered_vec;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "ModuleTargeted")]
pub enum Targeted {
    RookieLaser,
    RookieMiner,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "ModuleTargetedDetails")]
pub struct Details {
    pub required_cpu: u16,
    pub required_powergrid: u16,

    #[serde(serialize_with = "ordered_vec")]
    pub effects_origin: Vec<RoundEffect>,
    #[serde(serialize_with = "ordered_vec")]
    pub effects_target: Vec<RoundEffect>,
}

#[cfg(test)]
ts_rs::export! {
    Targeted => "module-targeted.ts",
    Details => "module-targeted-details.ts",
}
