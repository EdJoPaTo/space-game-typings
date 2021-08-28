#![allow(clippy::module_name_repetitions)]

use serde::{Deserialize, Serialize};

use crate::fixed::solarsystem::Solarsystem;
use crate::site::Site;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", untagged)]
pub enum PlayerLocation {
    Site(PlayerLocationSite),
    Station(PlayerLocationStation),
    Warp(PlayerLocationWarp),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct PlayerLocationSite {
    pub solarsystem: Solarsystem,
    pub site: Site,
}

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct PlayerLocationStation {
    pub solarsystem: Solarsystem,
    pub station: u8,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct PlayerLocationWarp {
    pub solarsystem: Solarsystem,
    pub towards: Site,
}

#[cfg(feature = "typescript")]
ts_rs::export! {
    PlayerLocationSite,
    PlayerLocationStation,
    PlayerLocationWarp,
    PlayerLocation => "typescript/generated-player-location.ts"
}

impl Default for PlayerLocation {
    fn default() -> Self {
        Self::Station(PlayerLocationStation::default())
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
    let data = PlayerLocation::Site(PlayerLocationSite {
        solarsystem: Solarsystem::default(),
        site: Site::Station(42),
    });
    crate::test_helper::can_serde_parse(&data);
}

#[test]
fn can_identify_warp() {
    let data = PlayerLocation::Warp(PlayerLocationWarp {
        solarsystem: Solarsystem::default(),
        towards: Site::Station(42),
    });
    crate::test_helper::can_serde_parse(&data);
}

#[test]
fn can_identify_station() {
    let data = PlayerLocation::Station(PlayerLocationStation {
        solarsystem: Solarsystem::default(),
        station: 2,
    });
    crate::test_helper::can_serde_parse(&data);
}
