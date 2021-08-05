use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::fixed::site::Kind;
use crate::fixed::solarsystem;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
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

impl Info {
    #[must_use]
    pub fn generate_station(solarsystem: &solarsystem::Identifier, station_index: u8) -> Self {
        let number = station_index + 1;
        // TODO: römisch
        let name = format!("{} {}", solarsystem, number);
        let unique = format!("station{}", number);
        Self {
            kind: Kind::Station,
            name: Some(name),
            unique,
        }
    }

    #[must_use]
    pub fn generate_stargate(target_solarsystem: &solarsystem::Identifier) -> Self {
        let name = target_solarsystem.to_string();
        let unique = format!("stargate{}", target_solarsystem);
        Self {
            kind: Kind::Stargate,
            name: Some(name),
            unique,
        }
    }
}

#[test]
fn can_deserialize_no_name() -> anyhow::Result<()> {
    let result = serde_json::from_str::<Info>(r#"{"kind": "stargate", "unique": "42"}"#)?;
    assert!(result.name.is_none());
    Ok(())
}
