use serde::{Deserialize, Serialize};

use super::facility;

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "SiteKind")]
pub enum Kind {
    Facility(facility::Identifier),
    AsteroidField,
}

#[cfg(test)]
ts_rs::export! {Kind => "site-kind.ts"}
