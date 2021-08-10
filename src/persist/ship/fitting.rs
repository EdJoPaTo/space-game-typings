use serde::{Deserialize, Serialize};

use crate::fixed::{module, shiplayout, Statics};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "ShipFitting")]
pub struct Fitting {
    pub layout: shiplayout::Identifier,

    pub slots_targeted: Vec<module::TargetedIdentifier>,
    pub slots_untargeted: Vec<module::UntargetedIdentifier>,
    pub slots_passive: Vec<module::PassiveIdentifier>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    LayoutNotExistant,

    Cpu((u16, u16)),
    Powergrid((u16, u16)),

    TooManyPassiveModules,
    TooManyTargetedModules,
    TooManyUntargetedModules,

    UnknownPassiveModule(String),
    UnknownTargetedModule(String),
    UnknownUntargetedModule(String),
}

#[cfg(test)]
ts_rs::export! {
    Fitting => "ship-fitting.ts",
}

impl Default for Fitting {
    fn default() -> Self {
        Self {
            layout: "shiplayoutRookieShip".into(),
            slots_targeted: vec!["modtRookieMiner".into(), "modtRookieLaser".into()],
            slots_untargeted: vec!["moduRookieArmorRepair".into()],
            slots_passive: vec!["modpRookieArmorPlate".into()],
        }
    }
}

impl Fitting {
    /// Check if the fitting is valid or not
    /// # Errors
    /// When Fitting isnt valid the Error states why
    pub fn is_valid(&self, statics: &Statics) -> Result<(), Error> {
        if let Some(layout) = statics.ship_layouts.get(&self.layout) {
            // More modules than layout offers
            if self.slots_targeted.len() > layout.slots_targeted.into() {
                return Err(Error::TooManyTargetedModules);
            }
            if self.slots_untargeted.len() > layout.slots_untargeted.into() {
                return Err(Error::TooManyUntargetedModules);
            }
            if self.slots_passive.len() > layout.slots_passive.into() {
                return Err(Error::TooManyPassiveModules);
            }

            let mut cpu = 0;
            let mut powergrid = 0;

            // Modules exist
            for id in &self.slots_targeted {
                if let Some(m) = statics.modules_targeted.get(id) {
                    cpu += m.required_cpu;
                    powergrid += m.required_powergrid;
                } else {
                    return Err(Error::UnknownTargetedModule(id.to_string()));
                }
            }
            for id in &self.slots_untargeted {
                if let Some(m) = statics.modules_untargeted.get(id) {
                    cpu += m.required_cpu;
                    powergrid += m.required_powergrid;
                } else {
                    return Err(Error::UnknownUntargetedModule(id.to_string()));
                }
            }
            for id in &self.slots_passive {
                if let Some(m) = statics.modules_passive.get(id) {
                    cpu += m.required_cpu;
                    powergrid += m.required_powergrid;
                } else {
                    return Err(Error::UnknownPassiveModule(id.to_string()));
                }
            }

            // Check cpu / powergrid
            if layout.cpu < cpu {
                return Err(Error::Cpu((cpu, layout.cpu)));
            }
            if layout.powergrid < powergrid {
                return Err(Error::Powergrid((powergrid, layout.powergrid)));
            }

            return Ok(());
        }
        Err(Error::LayoutNotExistant)
    }
}

#[test]
fn default_fitting_is_valid() {
    let statics = crate::fixed::Statics::default();
    assert_eq!(Fitting::default().is_valid(&statics), Ok(()));
}
