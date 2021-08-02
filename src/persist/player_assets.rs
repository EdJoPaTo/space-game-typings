use serde::{Deserialize, Serialize};

use crate::fixed::solarsystem;

use super::ship::Fitting;

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct PlayerStationAssets {
    pub solarsystem: solarsystem::Identifier,
    pub station: u8,

    pub ships: Vec<Fitting>,
}

#[cfg(test)]
ts_rs::export! {PlayerStationAssets => "player-station-assets.ts"}
