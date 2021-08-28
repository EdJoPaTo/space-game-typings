use serde::{Deserialize, Serialize};

use crate::serde_helper::is_default;

/// The current situation of the entity / ship.
/// For the totals check the `ShipFitting`.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct Collateral {
    pub structure: u16,

    #[serde(default, skip_serializing_if = "is_default")]
    pub armor: u16,

    #[serde(default, skip_serializing_if = "is_default")]
    pub capacitor: u16,
}

/// In Percentages 0.0..=1.0
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct Health {
    pub armor: f32,
    pub structure: f32,
}

impl Collateral {
    pub const DEAD: Collateral = Collateral {
        capacitor: 0,
        armor: 0,
        structure: 0,
    };

    /// New Collataral with only structure and nothing else.
    /// Helpful for initializing lifeless dumb thingies like asteroids.
    #[must_use]
    pub fn new_structure(structure: u16) -> Self {
        Self {
            structure,
            armor: 0,
            capacitor: 0,
        }
    }

    /// Returns the minimum of two collaterals.
    /// Helpful when ensuring a collateral is still within the ships limits
    /// # Example
    /// ```
    /// use space_game_typings::entity::Collateral;
    /// let status = Collateral {capacitor: 20, armor: 42, structure: 4};
    /// let ship_maximum = Collateral {capacitor: 20, armor: 20, structure: 10};
    /// let min = status.min(ship_maximum);
    /// assert_eq!(min.capacitor, 20);
    /// assert_eq!(min.armor, 20);
    /// assert_eq!(min.structure, 4);
    /// ```
    #[must_use]
    pub fn min(self, other: Self) -> Self {
        Self {
            capacitor: self.capacitor.min(other.capacitor),
            armor: self.armor.min(other.armor),
            structure: self.structure.min(other.structure),
        }
    }

    /// Returns if it's still alive or not
    /// # Example
    /// ```
    /// use space_game_typings::entity::Collateral;
    /// assert!(Collateral {capacitor: 20, armor: 42, structure: 4}.is_alive());
    /// assert!(Collateral {capacitor: 0, armor: 0, structure: 4}.is_alive());
    /// assert!(!Collateral {capacitor: 20, armor: 42, structure: 0}.is_alive());
    /// ```
    #[must_use]
    pub const fn is_alive(self) -> bool {
        self.structure > 0
    }

    #[must_use]
    pub fn calc_health_raw(self, max_armor: u16, max_structure: u16) -> Health {
        let armor = f32::from(self.armor) / f32::from(max_armor);
        let structure = f32::from(self.structure) / f32::from(max_structure);
        Health { armor, structure }
    }

    #[must_use]
    pub fn calc_health(self, max: Collateral) -> Health {
        self.calc_health_raw(max.armor, max.structure)
    }
}
