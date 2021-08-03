use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::fixed::site::Kind;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "SiteInfo")]
pub struct Info {
    pub kind: Kind,
    pub unique: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

pub type SitesNearPlanet = BTreeMap<u8, Vec<Info>>;

#[cfg(test)]
ts_rs::export! {
    Info => "site-info.ts",
}

#[test]
fn can_deserialize_no_name() -> anyhow::Result<()> {
    let result = serde_json::from_str::<Info>(r#"{"kind": "facilityStargate", "unique": "42"}"#)?;
    assert!(result.name.is_none());
    Ok(())
}
