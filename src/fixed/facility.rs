use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub enum Service {
    Dock,
    Jump,
}

#[derive(Debug, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "FacilityIdentifier")]
pub enum Identifier {
    #[serde(rename = "facilityStargate")]
    Stargate,
    #[serde(rename = "facilityStation")]
    Station,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct Facility {
    pub services: Vec<Service>,
}

#[cfg(test)]
ts_rs::export! {
    Service => "facility-service.ts",
    Identifier => "facility-identifier.ts",
    Facility => "facility.ts",
}
