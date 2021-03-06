#![allow(clippy::module_name_repetitions)]

use serde::{Deserialize, Serialize};

use crate::fixed::solarsystem::Solarsystem;
use crate::site::Site;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS), ts(export))]
#[serde(rename_all = "camelCase", untagged)]
pub enum PlayerLocation {
    Site(PlayerLocationSite),
    Station(PlayerLocationStation),
    Warp(PlayerLocationWarp),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS), ts(export))]
#[serde(rename_all = "camelCase")]
pub struct PlayerLocationSite {
    pub solarsystem: Solarsystem,
    pub site: Site,
}

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS), ts(export))]
#[serde(rename_all = "camelCase")]
pub struct PlayerLocationStation {
    pub solarsystem: Solarsystem,
    pub station: u8,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS), ts(export))]
#[serde(rename_all = "camelCase")]
pub struct PlayerLocationWarp {
    pub solarsystem: Solarsystem,
    pub towards: Site,
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
            Self::Site(o) => o.solarsystem,
            Self::Station(o) => o.solarsystem,
            Self::Warp(o) => o.solarsystem,
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
