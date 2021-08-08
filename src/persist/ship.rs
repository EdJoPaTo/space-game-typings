use serde::{Deserialize, Serialize};

use crate::fixed::{module, shiplayout, Statics};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct Ship {
    pub fitting: Fitting,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "ShipFitting")]
pub struct Fitting {
    pub layout: shiplayout::Identifier,

    pub slots_targeted: Vec<module::TargetedIdentifier>,
    pub slots_untargeted: Vec<module::UntargetedIdentifier>,
    pub slots_passive: Vec<module::PassiveIdentifier>,
}

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
    Ship => "ship.ts",
    Fitting => "ship-fitting.ts",
    Status => "ship-status.ts",
}

impl Ship {
    #[allow(clippy::missing_panics_doc)]
    #[must_use]
    pub fn default(statics: &Statics) -> Self {
        Self {
            fitting: Fitting::default(),
            status: Status::new(statics, &Fitting::default()).unwrap(),
        }
    }
}

impl Default for Fitting {
    fn default() -> Self {
        Self {
            layout: "shiplayoutRookieShip".into(),
            slots_targeted: vec!["modtRookieMiningLaser".into(), "modtRookieLaser".into()],
            slots_untargeted: vec!["moduRookieArmorRepair".into()],
            slots_passive: vec!["modpRookieArmorPlate".into()],
        }
    }
}

impl Fitting {
    #[must_use]
    pub fn is_valid(&self, statics: &Statics) -> bool {
        if let Some(layout) = statics.ship_layouts.get(&self.layout) {
            if self.slots_targeted.len() > layout.slots_targeted.into()
                || self.slots_untargeted.len() > layout.slots_untargeted.into()
                || self.slots_passive.len() > layout.slots_passive.into()
            {
                return false;
            }

            // TODO: check modules existing

            // TODO: check cpu / powergrid

            return true;
        }
        false
    }
}

impl Status {
    #[must_use]
    pub fn new(statics: &Statics, fitting: &Fitting) -> Option<Self> {
        let layout = statics.ship_layouts.get(&fitting.layout)?;
        let mut status = Status {
            capacitor: layout.capacitor,
            hitpoints_armor: layout.hitpoints_armor,
            hitpoints_structure: layout.hitpoints_structure,
        };
        for passive_identifier in &fitting.slots_passive {
            if let Some(module) = statics.modules_passive.get(passive_identifier) {
                if let Some(armor) = module.hitpoints_armor {
                    status.hitpoints_armor = status.hitpoints_armor.saturating_add(armor);
                }
                if let Some(capacitor) = module.capacitor {
                    status.capacitor = status.capacitor.saturating_add(capacitor);
                }
            }
        }
        Some(status)
    }

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
        let ship_maximum = Self::new(statics, fitting)?;
        Some(self.min(ship_maximum))
    }

    #[must_use]
    pub const fn is_alive(self) -> bool {
        self.hitpoints_structure > 0
    }
}

#[test]
fn can_generate_default_ship() {
    let statics = Statics::default();
    let ship = Ship::default(&statics);
    assert_eq!(ship.fitting.layout, Fitting::default().layout);
}

#[test]
fn default_fitting_is_valid() {
    let statics = crate::fixed::Statics::default();
    assert!(Fitting::default().is_valid(&statics));
}

#[test]
fn status_without_modules_correct() {
    let statics = Statics::default();
    let expected = statics.ship_layouts.get("shiplayoutFrigate").unwrap();
    let fitting = Fitting {
        layout: "shiplayoutFrigate".to_string(),
        slots_targeted: vec![],
        slots_untargeted: vec![],
        slots_passive: vec![],
    };
    let result = Status::new(&statics, &fitting);
    assert_eq!(
        result,
        Some(Status {
            capacitor: expected.capacitor,
            hitpoints_armor: expected.hitpoints_armor,
            hitpoints_structure: expected.hitpoints_structure,
        })
    );
}

#[test]
fn status_of_default_fitting_correct() {
    let statics = Statics::default();
    let fitting = Fitting::default();
    let expected_layout = statics.ship_layouts.get(&fitting.layout).unwrap();
    let expected_passive = statics
        .modules_passive
        .get(&fitting.slots_passive[0])
        .unwrap();
    let result = Status::new(&statics, &fitting);
    assert_eq!(
        result,
        Some(Status {
            capacitor: expected_layout.capacitor,
            hitpoints_armor: expected_layout.hitpoints_armor
                + expected_passive.hitpoints_armor.unwrap(),
            hitpoints_structure: expected_layout.hitpoints_structure,
        })
    );
}
