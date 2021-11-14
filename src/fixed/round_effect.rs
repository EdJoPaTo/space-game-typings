use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Hash, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS), ts(export))]
#[serde(rename_all = "camelCase", tag = "type", content = "amount")]
pub enum RoundEffect {
    CapacitorDrain(u16),
    CapacitorRecharge(u16),

    ArmorRepair(u16),
    /// Should only be used by the GuardianDefender
    StructureRepair(u16),

    Damage(u16),
    Mine(u32),
    WarpDisruption,
}
