use serde::{Deserialize, Serialize};

mod fitting;
mod status;

pub use fitting::Fitting;
pub use status::Status;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct Ship {
    pub fitting: Fitting,
    pub status: Status,
}

#[cfg(test)]
ts_rs::export! {
    Ship => "ship.ts",
    Status => "ship-status.ts",
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
        }
    }
}

#[cfg(test)]
use crate::fixed::Statics;

#[test]
fn default_ship_is_exactly_from_statics() {
    let statics = Statics::default();
    let expected = Ship {
        fitting: Fitting::default(),
        status: Status::new(&statics, &Fitting::default()).unwrap(),
    };

    assert_eq!(Ship::default().fitting, Fitting::default());
    assert_eq!(Ship::default().status, expected.status);
    assert_eq!(Ship::default(), expected);
}
