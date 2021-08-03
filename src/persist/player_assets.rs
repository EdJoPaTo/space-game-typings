use serde::{Deserialize, Serialize};

use super::ship::Fitting;

#[derive(Debug, Default, Serialize, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct PlayerStationAssets {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ships: Vec<Fitting>,
}

#[cfg(test)]
ts_rs::export! {PlayerStationAssets => "player-station-assets.ts"}

#[test]
fn can_deserialize_empty() -> anyhow::Result<()> {
    let result = serde_json::from_str::<PlayerStationAssets>("{}")?;
    assert!(result.ships.is_empty());
    Ok(())
}
