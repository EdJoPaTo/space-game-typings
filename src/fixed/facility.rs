use serde::{Deserialize, Serialize};

use crate::serde_helper::ordered_vec;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub enum Service {
    Dock,
    Jump,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "Facility")]
pub enum Facility {
    Station,
    Stargate,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "FacilityDetails")]
pub struct Details {
    #[serde(serialize_with = "ordered_vec")]
    pub services: Vec<Service>,
}

#[cfg(feature = "typescript")]
ts_rs::export! {
    Service,
    Details,
    Facility => "typescript/generated-facility.ts"
}
