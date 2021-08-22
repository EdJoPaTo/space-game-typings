use serde::{Deserialize, Serialize};

use crate::fixed::Statics;

use super::Fitting;

/// The current stuff carried with the ship.
/// For the totals check the `ShipFitting`.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "ShipCargo")]
pub struct Cargo {
    pub ore: u16,
}

/// The amounts of stuff a ship can carry.
/// This is used for math like 'how much is possible - currently loaded = free'
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct CargoAmounts {
    pub ore: u16,
}

impl Cargo {
    #[must_use]
    pub const fn current_amounts(&self) -> CargoAmounts {
        CargoAmounts { ore: self.ore }
    }

    #[must_use]
    pub fn free(&self, statics: &Statics, fitting: &Fitting) -> CargoAmounts {
        let max = CargoAmounts::maximum(statics, fitting);
        let current = self.current_amounts();
        max.saturating_sub(current)
    }

    #[must_use]
    pub fn add(&self, other: &Self) -> Self {
        Self {
            ore: self.ore.saturating_add(other.ore),
        }
    }
}

impl CargoAmounts {
    #[must_use]
    pub fn maximum(statics: &Statics, fitting: &Fitting) -> Self {
        let layout = statics.ship_layouts.get(&fitting.layout);
        Self {
            ore: layout.ore_bay,
        }
    }

    #[must_use]
    const fn saturating_sub(self, other: Self) -> Self {
        Self {
            ore: self.ore.saturating_sub(other.ore),
        }
    }
}
