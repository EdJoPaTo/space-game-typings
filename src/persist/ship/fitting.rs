use serde::{Deserialize, Serialize};

use crate::fixed::module::passive::Passive;
use crate::fixed::module::targeted::Targeted;
use crate::fixed::module::untargeted::Untargeted;
use crate::fixed::shiplayout::ShipLayout;
use crate::fixed::Statics;

use super::Status;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "ShipFitting")]
pub struct Fitting {
    pub layout: ShipLayout,

    pub slots_targeted: Vec<Targeted>,
    pub slots_untargeted: Vec<Untargeted>,
    pub slots_passive: Vec<Passive>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    Cpu((u16, u16)),
    Powergrid((u16, u16)),
    HitpointsStructureZero,

    TooManyPassiveModules,
    TooManyTargetedModules,
    TooManyUntargetedModules,
}

#[cfg(test)]
ts_rs::export! {
    Fitting => "ship-fitting.ts",
}

impl Default for Fitting {
    fn default() -> Self {
        Self {
            layout: ShipLayout::RookieShip,
            slots_targeted: vec![Targeted::RookieMiner, Targeted::RookieLaser],
            slots_untargeted: vec![Untargeted::RookieArmorRepair],
            slots_passive: vec![Passive::RookieArmorPlate],
        }
    }
}

impl Fitting {
    /// Check if the fitting is valid or not
    /// # Errors
    /// When Fitting isnt valid the Error states why
    pub fn is_valid(&self, statics: &Statics) -> Result<(), Error> {
        let layout = statics
            .ship_layouts
            .get(&self.layout)
            .expect("statics have to contain ship layout");
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
        for id in &self.slots_targeted {
            let m = statics
                .modules_targeted
                .get(id)
                .expect("targeted module has to be in statics");
            cpu += m.required_cpu;
            powergrid += m.required_powergrid;
        }
        for id in &self.slots_untargeted {
            let m = statics
                .modules_untargeted
                .get(id)
                .expect("untargeted module has to be in statics");
            cpu += m.required_cpu;
            powergrid += m.required_powergrid;
        }
        for id in &self.slots_passive {
            let m = statics
                .modules_passive
                .get(id)
                .expect("passive module has to be in statics");
            cpu += m.required_cpu;
            powergrid += m.required_powergrid;
        }

        // Check cpu / powergrid
        if cpu > layout.cpu {
            return Err(Error::Cpu((cpu, layout.cpu)));
        }
        if powergrid > layout.powergrid {
            return Err(Error::Powergrid((powergrid, layout.powergrid)));
        }

        // Dead on undock
        let status = self.maximum_status(statics);
        if !status.is_alive() {
            return Err(Error::HitpointsStructureZero);
        }

        Ok(())
    }

    #[must_use]
    #[allow(clippy::cast_sign_loss)]
    pub fn maximum_status(&self, statics: &Statics) -> Status {
        #[allow(clippy::cast_sign_loss)]
        const fn add(base: u16, add: i16) -> u16 {
            if add >= 0 {
                base.saturating_add(add as u16)
            } else {
                let b = add.saturating_abs() as u16;
                base.saturating_sub(b)
            }
        }

        let mut status = statics
            .ship_layouts
            .get(&self.layout)
            .expect("statics have to contain ship layout")
            .status;

        for id in &self.slots_passive {
            let m = statics
                .modules_passive
                .get(id)
                .expect("passive module has to be in statics");
            status.hitpoints_armor = add(status.hitpoints_armor, m.hitpoints_armor);
        }

        status
    }
}

#[test]
fn default_fitting_is_valid() {
    let statics = crate::fixed::Statics::default();
    assert_eq!(Fitting::default().is_valid(&statics), Ok(()));
}

#[test]
#[allow(clippy::cast_sign_loss)]
fn status_without_modules_correct() {
    let statics = Statics::default();
    let expected = statics.ship_layouts.get(&ShipLayout::RookieShip).unwrap();
    let fitting = Fitting {
        layout: ShipLayout::RookieShip,
        slots_targeted: vec![],
        slots_untargeted: vec![],
        slots_passive: vec![],
    };
    let result = fitting.maximum_status(&statics);
    assert_eq!(result, expected.status);
}

#[test]
#[allow(clippy::cast_sign_loss)]
fn status_of_default_fitting_correct() {
    let statics = Statics::default();
    let fitting = Fitting::default();
    let expected_layout = statics.ship_layouts.get(&fitting.layout).unwrap();
    let expected_passive = statics
        .modules_passive
        .get(&fitting.slots_passive[0])
        .unwrap();
    let result = fitting.maximum_status(&statics);
    assert_eq!(
        result,
        Status {
            capacitor: expected_layout.status.capacitor,
            hitpoints_armor: (expected_layout.status.hitpoints_armor)
                + (expected_passive.hitpoints_armor as u16),
            hitpoints_structure: expected_layout.status.hitpoints_structure,
        }
    );
}
