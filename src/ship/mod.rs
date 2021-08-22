use serde::{Deserialize, Serialize};

use crate::entity::{Collateral, Health};
use crate::fixed::Statics;

mod cargo;
mod fitting;

pub use cargo::{Cargo, CargoAmounts};
pub use fitting::Fitting;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct Ship {
    pub fitting: Fitting,
    pub collateral: Collateral,
    pub cargo: Cargo,
}

#[cfg(feature = "typescript")]
ts_rs::export! {
    Cargo => "ship-cargo.ts",
    CargoAmounts => "ship-cargo-amounts.ts",
    Fitting => "ship-fitting.ts",
    Ship => "ship.ts",
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
            cargo: Cargo::default(),
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
            cargo: Cargo::default(),
        }
    }

    #[must_use]
    pub fn to_health(&self, statics: &Statics) -> Health {
        let max = self.fitting.maximum_collateral(statics);
        self.collateral.calc_health(max)
    }

    #[must_use]
    pub fn free_cargo(&self, statics: &Statics) -> CargoAmounts {
        self.cargo.free(statics, &self.fitting)
    }
}

#[test]
fn default_ship_is_exactly_from_statics() {
    let statics = Statics::default();
    let expected = Ship {
        fitting: Fitting::default(),
        collateral: Fitting::default().maximum_collateral(&statics),
        cargo: Cargo::default(),
    };

    assert_eq!(Ship::default().fitting, Fitting::default());
    assert_eq!(Ship::default().collateral, expected.collateral);
    assert_eq!(Ship::default().cargo, expected.cargo);
    assert_eq!(Ship::default(), expected);
}
