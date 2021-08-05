use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "SiteKind")]
pub enum Kind {
    FacilityStargate,
    FacilityStation,
    AsteroidField,
}

#[cfg(test)]
ts_rs::export! {Kind => "site-kind.ts"}
