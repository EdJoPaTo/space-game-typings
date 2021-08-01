use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::fixed::site::Kind;

use super::site_entity::SiteEntity;

// TODO: the frontend dont care for internals like fittings.
// Split up typings and have one internal and one external?
// For now… just share everything witht the frontend.

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "SiteInfo")]
pub struct Info {
    pub kind: Kind,
    pub unique: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", rename = "SiteInners")]
pub struct Inners {
    pub entities: Vec<SiteEntity>,
}

pub type SitesNearPlanet = BTreeMap<u8, Vec<Info>>;

#[cfg(test)]
ts_rs::export! {
    Info => "site-info.ts",
}
