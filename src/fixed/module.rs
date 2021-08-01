use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "ModulePassive")]
pub struct Passive {
    pub required_cpu: u32,
    pub required_powergrid: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacitor: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hitpoints_armor: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "ModuleUntargeted")]
pub struct Untargeted {
    pub required_cpu: u32,
    pub required_powergrid: u32,

    pub energy_consumption: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub armor_repair: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "ModuleTargeted")]
pub struct Targeted {
    pub required_cpu: u32,
    pub required_powergrid: u32,

    pub energy_consumption: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_mined: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub damage: Option<u32>,
}

#[cfg(test)]
ts_rs::export! {
    Passive => "module-passive.ts",
    Untargeted => "module-untargeted.ts",
    Targeted => "module-targeted.ts",
}
