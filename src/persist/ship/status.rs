use serde::{Deserialize, Serialize};

use crate::fixed::Statics;

use super::Fitting;

/// The current situation of the ship.
/// For the totals check the `ShipFitting`.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "ShipStatus")]
pub struct Status {
    pub capacitor: u16,
    pub hitpoints_armor: u16,
    pub hitpoints_structure: u16,
}

#[cfg(test)]
ts_rs::export! {
    Status => "ship-status.ts",
}

impl Status {
    #[must_use]
    /// Returns the minimum of two status thingies.
    /// Helpful when ensuring a status is still within the ships limits
    /// # Example
    /// ```
    /// use typings::persist::ship::Status;
    /// let status = Status {capacitor: 20, hitpoints_armor: 42, hitpoints_structure: 4};
    /// let ship_maximum = Status {capacitor: 20, hitpoints_armor: 20, hitpoints_structure: 10};
    /// let min = status.min(ship_maximum);
    /// assert_eq!(min.capacitor, 20);
    /// assert_eq!(min.hitpoints_armor, 20);
    /// assert_eq!(min.hitpoints_structure, 4);
    /// ```
    pub fn min(self, other: Self) -> Self {
        Self {
            capacitor: self.capacitor.min(other.capacitor),
            hitpoints_armor: self.hitpoints_armor.min(other.hitpoints_armor),
            hitpoints_structure: self.hitpoints_structure.min(other.hitpoints_structure),
        }
    }

    #[must_use]
    /// Returns the possible status in this fitting.
    pub fn min_layout(self, statics: &Statics, fitting: &Fitting) -> Option<Self> {
        Some(self.min(fitting.maximum_status(statics)))
    }

    #[must_use]
    pub const fn is_alive(self) -> bool {
        self.hitpoints_structure > 0
    }

    #[must_use]
    /// (Armor, Structure) within 0.0..=1.0
    pub fn health_percentage(self, max: (u16, u16)) -> (f32, f32) {
        let armor = f32::from(self.hitpoints_armor) / f32::from(max.0);
        let structure = f32::from(self.hitpoints_structure) / f32::from(max.1);
        (armor, structure)
    }

    #[must_use]
    /// (Armor, Structure) within 0.0..=1.0
    pub fn health_percentage_layout(
        self,
        statics: &Statics,
        fitting: &Fitting,
    ) -> Option<(f32, f32)> {
        let max = fitting.maximum_status(statics);
        Some(self.health_percentage((max.hitpoints_armor, max.hitpoints_structure)))
    }
}
