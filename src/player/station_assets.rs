use serde::{Deserialize, Serialize};

use crate::ship::Ship;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "PlayerStationAssets")]
pub struct StationAssets {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ships: Vec<Ship>,
}

#[cfg(feature = "typescript")]
ts_rs::export! {
    StationAssets => "player-station-assets.ts",
}

#[test]
fn can_deserialize_empty() -> anyhow::Result<()> {
    let result = serde_json::from_str::<StationAssets>("{}")?;
    assert!(result.ships.is_empty());
    Ok(())
}
