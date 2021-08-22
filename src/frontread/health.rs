use serde::{Deserialize, Serialize};

use crate::fixed::Statics;
use crate::ship::{Ship, Status};

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
/// In Percentages 0.0..=1.0
pub struct Health {
    pub armor: f32,
    pub structure: f32,
}

#[cfg(feature = "typescript")]
ts_rs::export! {
    Health => "health.ts",
}

impl Health {
    #[must_use]
    pub fn from_raw(current: Status, max_armor: u16, max_structure: u16) -> Self {
        let armor = f32::from(current.hitpoints_armor) / f32::from(max_armor);
        let structure = f32::from(current.hitpoints_structure) / f32::from(max_structure);
        Self { armor, structure }
    }

    #[must_use]
    pub fn from_status(current: Status, max: Status) -> Self {
        Self::from_raw(current, max.hitpoints_armor, max.hitpoints_structure)
    }

    #[must_use]
    pub fn from_ship(statics: &Statics, ship: &Ship) -> Self {
        let max = ship.fitting.maximum_status(statics);
        Self::from_status(ship.status, max)
    }
}
