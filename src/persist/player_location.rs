use serde::{Deserialize, Serialize};

use crate::fixed::solarsystem;

use super::site::Info;

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", untagged)]
pub enum PlayerLocation {
    Site(Site),
    Station(Station),
    Warp(Warp),
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
}

#[derive(Debug, Serialize, Deserialize)]
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
fn can_identify_site() -> anyhow::Result<()> {
    let data = PlayerLocation::Site(Site {
        solarsystem: "bla".to_string(),
        site: Info {
            kind: crate::fixed::site::Kind::AsteroidField,
            unique: "666".to_string(),
            name: None,
        },
    });
    let json = serde_json::to_string_pretty(&data)?;
    println!("json {}", json);
    let some = serde_json::from_str::<PlayerLocation>(&json)?;
    println!("parsed {:?}", some);
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
    });
    let json = serde_json::to_string_pretty(&data)?;
    println!("json {}", json);
    let some = serde_json::from_str::<PlayerLocation>(&json)?;
    println!("parsed {:?}", some);
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
    println!("parsed {:?}", some);
    if let PlayerLocation::Station(_) = some {
        Ok(())
    } else {
        panic!("wrong!");
    }
}
