use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub enum NpcFaction {
    Guards,
    Pirates,
}

#[cfg(test)]
ts_rs::export! {NpcFaction => "npc-faction.ts"}
