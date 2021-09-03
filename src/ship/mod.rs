use serde::{Deserialize, Serialize};

use crate::entity::{Collateral, Health};
use crate::fixed::Statics;
use crate::serde_helper::is_default;
use crate::storage::Storage;

mod fitting;

pub use fitting::{
    Error as FittingError, Fitting, InfrastructureUsage as FittingInfrastructureUsage,
};

// TODO: remove Clone in order to ensure Ships are unique and only get moved?

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct Ship {
    pub fitting: Fitting,
    pub collateral: Collateral,

    #[serde(default, skip_serializing_if = "is_default")]
    pub cargo: Storage,
}

#[cfg(feature = "typescript")]
ts_rs::export! {
    Fitting,
    FittingInfrastructureUsage,
    Ship => "typescript/generated-ship.ts"
}

impl Default for Ship {
    fn default() -> Self {
        Self {
            fitting: Fitting::default(),
            collateral: Collateral {
                capacitor: 40,
                armor: 30,
                structure: 10,
            },
            cargo: Storage::default(),
        }
    }
}

impl Ship {
    #[must_use]
    pub fn new(statics: &Statics, fitting: Fitting) -> Self {
        let collateral = fitting.maximum_collateral(statics);
        Self {
            fitting,
            collateral,
            cargo: Storage::default(),
        }
    }

    #[must_use]
    pub fn to_health(&self, statics: &Statics) -> Health {
        let max = self.fitting.maximum_collateral(statics);
        self.collateral.calc_health(max)
    }

    #[must_use]
    pub fn free_cargo(&self, statics: &Statics) -> u32 {
        let details = statics.ship_layouts.get(&self.fitting.layout);
        details.cargo_slots.saturating_sub(self.cargo.total_slots())
    }
}

#[test]
fn default_ship_is_exactly_from_statics() {
    let statics = Statics::default();
    let expected = Ship {
        fitting: Fitting::default(),
        collateral: Fitting::default().maximum_collateral(&statics),
        cargo: Storage::default(),
    };

    assert_eq!(Ship::default().fitting, Fitting::default());
    assert_eq!(Ship::default().collateral, expected.collateral);
    assert_eq!(Ship::default().cargo, expected.cargo);
    assert_eq!(Ship::default(), expected);
}
