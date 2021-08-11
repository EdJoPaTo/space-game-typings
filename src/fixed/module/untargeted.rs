use serde::{Deserialize, Serialize};

use crate::serde_helper::ordered_vec;

use super::Effect;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "ModuleUntargeted")]
pub enum Untargeted {
    RookieArmorRepair,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "ModuleUntargetedDetails")]
pub struct Details {
    pub required_cpu: u16,
    pub required_powergrid: u16,

    #[serde(serialize_with = "ordered_vec")]
    pub effects: Vec<Effect>,
}

#[cfg(test)]
ts_rs::export! {
    Untargeted => "module-untargeted.ts",
    Details => "module-untargeted-details.ts",
}
