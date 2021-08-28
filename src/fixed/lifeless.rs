use serde::{Deserialize, Serialize};

use crate::entity::Collateral;

use super::item::Ore;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "Lifeless")]
pub enum Lifeless {
    Asteroid,
    SmallWreck,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "LifelessDetails")]
pub struct Details {
    #[serde(flatten)]
    pub collateral: Collateral,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minable: Option<(Ore, u32)>,
    // TODO: lootable resources
    // TODO: hackable resources
}

#[cfg(feature = "typescript")]
ts_rs::export! {
    Lifeless => "lifeless.ts",
    Details => "lifeless-details.ts",
}
