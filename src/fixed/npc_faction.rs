use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub enum NpcFaction {
    Guards,
    Pirates,
}

#[cfg(feature = "typescript")]
ts_rs::export! {
    NpcFaction => "npc-faction.ts",
}
