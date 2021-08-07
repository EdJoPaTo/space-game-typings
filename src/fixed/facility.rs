use serde::{Deserialize, Serialize};

use crate::serde_helper::ordered_vec;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub enum Service {
    Dock,
    Jump,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "FacilityIdentifier")]
pub enum Identifier {
    Station,
    Stargate,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct Facility {
    #[serde(serialize_with = "ordered_vec")]
    pub services: Vec<Service>,
}

#[cfg(test)]
ts_rs::export! {
    Service => "facility-service.ts",
    Identifier => "facility-identifier.ts",
    Facility => "facility.ts",
}
