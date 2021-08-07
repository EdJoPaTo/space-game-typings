use serde::{Deserialize, Serialize};

use crate::fixed::{module, shiplayout, ShipLayouts, Statics};

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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
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
    pub fn new(layouts: &ShipLayouts, fitting: &Fitting) -> Option<Self> {
        let layout = layouts.get(&fitting.layout)?;
        Some(Self {
            capacitor: layout.capacitor,
            hitpoints_armor: layout.hitpoints_armor,
            hitpoints_structure: layout.hitpoints_structure,
        })
    }
}

#[test]
fn default_fitting_is_valid() {
    if let Ok(statics) = crate::fixed::Statics::import_yaml("static") {
        assert!(Fitting::default().is_valid(&statics));
    }
}
