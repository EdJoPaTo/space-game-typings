use serde::{Deserialize, Serialize};

use crate::fixed::Statics;

use super::Fitting;

/// The current situation of the ship.
/// For the totals check the `ShipFitting`.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "ShipStatus")]
pub struct Status {
    pub capacitor: u16,
    pub hitpoints_armor: u16,
    pub hitpoints_structure: u16,
}

#[cfg(feature = "typescript")]
ts_rs::export! {
    Status => "ship-status.ts",
}

impl Status {
    pub const DEAD: Status = Status {
        capacitor: 0,
        hitpoints_armor: 0,
        hitpoints_structure: 0,
    };

    #[must_use]
    /// Returns the minimum of two status thingies.
    /// Helpful when ensuring a status is still within the ships limits
    /// # Example
    /// ```
    /// use space_game_typings::persist::ship::Status;
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
    pub fn min_layout(self, statics: &Statics, fitting: &Fitting) -> Self {
        self.min(fitting.maximum_status(statics))
    }

    #[must_use]
    /// Returns if it's still alive or not
    /// # Example
    /// ```
    /// use space_game_typings::persist::ship::Status;
    /// assert!(Status {capacitor: 20, hitpoints_armor: 42, hitpoints_structure: 4}.is_alive());
    /// assert!(Status {capacitor: 0, hitpoints_armor: 0, hitpoints_structure: 4}.is_alive());
    /// assert!(!Status {capacitor: 20, hitpoints_armor: 42, hitpoints_structure: 0}.is_alive());
    /// ```
    pub const fn is_alive(self) -> bool {
        self.hitpoints_structure > 0
    }
}
