use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Hash, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", tag = "type", content = "amount")]
pub enum RoundEffect {
    CapacitorDrain(u16),
    CapacitorRecharge(u16),

    ArmorRepair(u16),

    Damage(u16),
    Mine(u16),
    WarpDisruption,
}

#[cfg(test)]
ts_rs::export! {
    RoundEffect => "round-effect.ts",
}
