use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct PlayerGeneral {
    pub home_solarsystem: String,
    pub home_station: u8,

    /// Paperclips are the currency
    pub paperclips: u64,
}

#[cfg(test)]
ts_rs::export! {PlayerGeneral => "player-general.ts"}
