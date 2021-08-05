use serde::{Deserialize, Serialize};

use crate::fixed::solarsystem;

use super::site::Info;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", untagged)]
pub enum PlayerLocation {
    Site(Site),
    Station(Station),
    Warp(Warp),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "PlayerLocationStation")]
pub struct Station {
    pub solarsystem: solarsystem::Identifier,
    pub station: u8,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "PlayerLocationWarp")]
pub struct Warp {
    pub solarsystem: solarsystem::Identifier,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "PlayerLocationSite")]
pub struct Site {
    pub solarsystem: solarsystem::Identifier,
    pub site: Info,
}

#[cfg(test)]
ts_rs::export! {
    PlayerLocation => "player-location.ts",
    Station => "player-location-station.ts",
    Warp => "player-location-warp.ts",
    Site => "player-location-site.ts",
}

#[test]
fn can_identify_site() {
    let data = PlayerLocation::Site(Site {
        solarsystem: "bla".to_string(),
        site: Info {
            kind: crate::fixed::site::Kind::AsteroidField,
            unique: "666".to_string(),
            name: None,
        },
    });
    crate::test_helper::can_serde_parse(&data);
}

#[test]
fn can_identify_warp() {
    let data = PlayerLocation::Warp(Warp {
        solarsystem: "bla".to_string(),
    });
    crate::test_helper::can_serde_parse(&data);
}

#[test]
fn can_identify_station() {
    let data = PlayerLocation::Station(Station {
        solarsystem: "bla".to_string(),
        station: 2,
    });
    crate::test_helper::can_serde_parse(&data);
}
