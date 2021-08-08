use serde::{Deserialize, Serialize};

use crate::fixed::solarsystem;

use super::site;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", untagged)]
pub enum PlayerLocation {
    Site(site::Identifier),
    Station(Station),
    Warp(Warp),
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "PlayerLocationStation")]
pub struct Station {
    pub solarsystem: solarsystem::Identifier,
    pub station: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "PlayerLocationWarp")]
pub struct Warp {
    pub solarsystem: solarsystem::Identifier,
    pub towards_site_unique: String,
}

#[cfg(test)]
ts_rs::export! {
    PlayerLocation => "player-location.ts",
    Station => "player-location-station.ts",
    Warp => "player-location-warp.ts",
}

impl Default for PlayerLocation {
    fn default() -> Self {
        Self::Station(Station::default())
    }
}

impl PlayerLocation {
    #[must_use]
    pub const fn solarsystem(&self) -> solarsystem::Identifier {
        match self {
            PlayerLocation::Site(o) => o.solarsystem,
            PlayerLocation::Station(o) => o.solarsystem,
            PlayerLocation::Warp(o) => o.solarsystem,
        }
    }
}

#[test]
fn can_identify_site() {
    let data = PlayerLocation::Site(site::Identifier {
        solarsystem: solarsystem::Identifier::default(),
        site_unique: "666".to_string(),
    });
    crate::test_helper::can_serde_parse(&data);
}

#[test]
fn can_identify_warp() {
    let data = PlayerLocation::Warp(Warp {
        solarsystem: solarsystem::Identifier::default(),
        towards_site_unique: "666".to_string(),
    });
    crate::test_helper::can_serde_parse(&data);
}

#[test]
fn can_identify_station() {
    let data = PlayerLocation::Station(Station {
        solarsystem: solarsystem::Identifier::default(),
        station: 2,
    });
    crate::test_helper::can_serde_parse(&data);
}
