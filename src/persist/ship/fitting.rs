use serde::{Deserialize, Serialize};

use crate::fixed::module::passive::Passive;
use crate::fixed::module::targeted::Targeted;
use crate::fixed::module::untargeted::Untargeted;
use crate::fixed::shiplayout::{ShipLayout, ShipQualities, ShipQuality};
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
    LayoutNotExistant,

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
            if layout.cpu < cpu {
                return Err(Error::Cpu((cpu, layout.cpu)));
            }
            if layout.powergrid < powergrid {
                return Err(Error::Powergrid((powergrid, layout.powergrid)));
            }

            // Dead on undock
            let status = self.maximum_status(statics);
            if !status.is_alive() {
                return Err(Error::HitpointsStructureZero);
            }

            return Ok(());
        }
        Err(Error::LayoutNotExistant)
    }

    #[must_use]
    pub fn qualities(&self, statics: &Statics) -> ShipQualities {
        let mut map = ShipQualities::new();
        let layout = statics
            .ship_layouts
            .get(&self.layout)
            .expect("ship_layout has to exist");
        for (q, amount) in &layout.qualities {
            let e = map.entry(*q).or_default();
            *e = e.saturating_add(*amount);
        }
        for id in &self.slots_passive {
            if let Some(m) = statics.modules_passive.get(id) {
                for (q, amount) in &m.qualities {
                    let e = map.entry(*q).or_default();
                    *e = e.saturating_add(*amount);
                }
            }
        }
        map
    }

    #[must_use]
    #[allow(clippy::cast_sign_loss)]
    pub fn maximum_status(&self, statics: &Statics) -> Status {
        let qualities = self.qualities(statics);
        Status {
            capacitor: qualities
                .get(&ShipQuality::Capacitor)
                .expect("ship has to have a capacitor")
                .saturating_abs() as u16,
            hitpoints_armor: qualities
                .get(&ShipQuality::HitpointsArmor)
                .map(|o| o.saturating_abs() as u16)
                .unwrap_or_default(),
            hitpoints_structure: qualities
                .get(&ShipQuality::HitpointsStructure)
                .expect("ship has to have structure")
                .saturating_abs() as u16,
        }
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
    assert_eq!(
        result,
        Status {
            capacitor: *expected.qualities.get(&ShipQuality::Capacitor).unwrap() as u16,
            hitpoints_armor: *expected
                .qualities
                .get(&ShipQuality::HitpointsArmor)
                .unwrap() as u16,
            hitpoints_structure: *expected
                .qualities
                .get(&ShipQuality::HitpointsStructure)
                .unwrap() as u16,
        }
    );
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
    let expected_passive_armor_bonus = 10;
    assert_eq!(
        expected_passive.qualities.values().collect::<Vec<_>>(),
        vec![&expected_passive_armor_bonus]
    );
    let result = fitting.maximum_status(&statics);
    assert_eq!(
        result,
        Status {
            capacitor: *expected_layout
                .qualities
                .get(&ShipQuality::Capacitor)
                .unwrap() as u16,
            hitpoints_armor: (*expected_layout
                .qualities
                .get(&ShipQuality::HitpointsArmor)
                .unwrap() as u16)
                + (expected_passive_armor_bonus as u16),
            hitpoints_structure: *expected_layout
                .qualities
                .get(&ShipQuality::HitpointsStructure)
                .unwrap() as u16,
        }
    );
}
