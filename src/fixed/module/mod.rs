use serde::{Deserialize, Serialize};

pub mod passive;
pub mod targeted;
pub mod untargeted;

#[derive(Debug, Clone, Copy, Serialize, Hash, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(
    rename_all = "camelCase",
    rename = "ModuleEffect",
    tag = "type",
    content = "amount"
)]
pub enum Effect {
    Capacitor(i16),
    ArmorRepair(u16),
    Damage(u16),
    Mine(u16),
    WarpDisruption,
}

#[cfg(test)]
ts_rs::export! {
    Effect => "module-effect.ts",
}
