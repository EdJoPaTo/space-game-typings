use serde::{Deserialize, Serialize};

use crate::fixed::Statics;

mod cargo;
mod fitting;
mod status;

pub use cargo::{Cargo, CargoAmounts};
pub use fitting::Fitting;
pub use status::Status;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct Ship {
    pub fitting: Fitting,
    pub status: Status,
    pub cargo: Cargo,
}

#[cfg(feature = "typescript")]
ts_rs::export! {
    Ship => "ship.ts",
}

impl Default for Ship {
    fn default() -> Self {
        Self {
            fitting: Fitting::default(),
            status: Status {
                capacitor: 40,
                hitpoints_armor: 30,
                hitpoints_structure: 10,
            },
            cargo: Cargo::default(),
        }
    }
}

impl Ship {
    #[must_use]
    pub fn new(statics: &Statics, fitting: Fitting) -> Self {
        let status = fitting.maximum_status(statics);
        Self {
            fitting,
            status,
            cargo: Cargo::default(),
        }
    }
}

#[test]
fn default_ship_is_exactly_from_statics() {
    let statics = Statics::default();
    let expected = Ship {
        fitting: Fitting::default(),
        status: Fitting::default().maximum_status(&statics),
        cargo: Cargo::default(),
    };

    assert_eq!(Ship::default().fitting, Fitting::default());
    assert_eq!(Ship::default().status, expected.status);
    assert_eq!(Ship::default().cargo, expected.cargo);
    assert_eq!(Ship::default(), expected);
}
