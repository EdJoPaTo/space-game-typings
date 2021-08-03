use serde::{Deserialize, Serialize};

use crate::fixed::solarsystem;

use super::ship::{Fitting, Status};
use super::site::Info;

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", untagged)]
pub enum PlayerLocation {
    Site(Site),
    Warp(Warp),
    Station(Station),
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "PlayerLocationStation")]
pub struct Station {
    pub solarsystem: solarsystem::Identifier,
    pub station: u8,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "PlayerLocationWarp")]
pub struct Warp {
    pub solarsystem: solarsystem::Identifier,

    pub ship_fitting: Fitting,
    pub ship_status: Status,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "PlayerLocationSite")]
pub struct Site {
    pub solarsystem: solarsystem::Identifier,
    pub site: Info,

    pub ship_fitting: Fitting,
    pub ship_status: Status,
}

#[cfg(test)]
ts_rs::export! {
    PlayerLocation => "player-location.ts",
    Station => "player-location-station.ts",
    Warp => "player-location-warp.ts",
    Site => "player-location-site.ts",
}

#[cfg(test)]
fn dummy_status() -> Status {
    Status {
        capacitor: 42,
        hitpoints_armor: 42,
        hitpoints_structure: 42,
    }
}

#[test]
fn can_identify_site() -> anyhow::Result<()> {
    let data = PlayerLocation::Site(Site {
        solarsystem: "bla".to_string(),
        site: Info {
            kind: crate::fixed::site::Kind::AsteroidField,
            unique: "666".to_string(),
            name: None,
        },
        ship_fitting: Fitting::default(),
        ship_status: dummy_status(),
    });
    let json = serde_json::to_string_pretty(&data)?;
    println!("json {}", json);
    let some = serde_json::from_str::<PlayerLocation>(&json)?;
    if let PlayerLocation::Site(_) = some {
        Ok(())
    } else {
        panic!("wrong!");
    }
}

#[test]
fn can_identify_warp() -> anyhow::Result<()> {
    let data = PlayerLocation::Warp(Warp {
        solarsystem: "bla".to_string(),
        ship_fitting: Fitting::default(),
        ship_status: dummy_status(),
    });
    let json = serde_json::to_string_pretty(&data)?;
    println!("json {}", json);
    let some = serde_json::from_str::<PlayerLocation>(&json)?;
    if let PlayerLocation::Warp(_) = some {
        Ok(())
    } else {
        panic!("wrong!");
    }
}

#[test]
fn can_identify_station() -> anyhow::Result<()> {
    let data = PlayerLocation::Station(Station {
        solarsystem: "bla".to_string(),
        station: 2,
    });
    let json = serde_json::to_string_pretty(&data)?;
    println!("json {}", json);
    let some = serde_json::from_str::<PlayerLocation>(&json)?;
    if let PlayerLocation::Station(_) = some {
        Ok(())
    } else {
        panic!("wrong!");
    }
}
