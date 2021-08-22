use serde::{Deserialize, Serialize};

use crate::ship::Fitting;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct PlayerStationAssets {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ships: Vec<Fitting>,
}

#[cfg(feature = "typescript")]
ts_rs::export! {
    PlayerStationAssets => "player-station-assets.ts",
}

#[test]
fn can_deserialize_empty() -> anyhow::Result<()> {
    let result = serde_json::from_str::<PlayerStationAssets>("{}")?;
    assert!(result.ships.is_empty());
    Ok(())
}
