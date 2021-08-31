use serde::{Deserialize, Serialize};

use crate::serde_helper::is_default;
use crate::ship::Ship;
use crate::storage::Storage;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "PlayerStationAssets")]
pub struct StationAssets {
    /// When the player is docked this ship is the ship the player sits in.
    /// It will be used to undock and when interacting with stuff like repair.
    /// The player can switch the ship and use one of the others in the station instead.
    ///
    /// When the player is docked None should be assumed as the default ship
    /// This can happen when the player exploded.
    /// When the player is not docked None is just "no one is docked anyway".
    ///
    /// This could have been realized as a pointer into the ships Vector but the are more things that can go wrong that way.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_ship: Option<Ship>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ships: Vec<Ship>,

    #[serde(default, skip_serializing_if = "is_default")]
    pub storage: Storage,
}

#[test]
fn can_deserialize_empty() -> anyhow::Result<()> {
    let result = serde_json::from_str::<StationAssets>("{}")?;
    assert!(result.ships.is_empty());
    Ok(())
}

#[test]
fn can_serde_parse_empty() {
    let data = StationAssets::default();
    crate::test_helper::can_serde_parse(&data);
}
