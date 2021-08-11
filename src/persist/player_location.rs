use serde::{Deserialize, Serialize};

use crate::fixed::solarsystem::Solarsystem;

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
    pub solarsystem: Solarsystem,
    pub station: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "PlayerLocationWarp")]
pub struct Warp {
    pub solarsystem: Solarsystem,
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
    pub const fn solarsystem(&self) -> Solarsystem {
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
        solarsystem: Solarsystem::default(),
        site_unique: "666".to_string(),
    });
    crate::test_helper::can_serde_parse(&data);
}

#[test]
fn can_identify_warp() {
    let data = PlayerLocation::Warp(Warp {
        solarsystem: Solarsystem::default(),
        towards_site_unique: "666".to_string(),
    });
    crate::test_helper::can_serde_parse(&data);
}

#[test]
fn can_identify_station() {
    let data = PlayerLocation::Station(Station {
        solarsystem: Solarsystem::default(),
        station: 2,
    });
    crate::test_helper::can_serde_parse(&data);
}
