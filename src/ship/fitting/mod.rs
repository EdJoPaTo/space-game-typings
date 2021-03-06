use serde::{Deserialize, Serialize};

use crate::fixed::module::{Passive, Targeted, Untargeted};
use crate::fixed::shiplayout::ShipLayout;
use crate::fixed::Statics;

use super::Collateral;

mod error;

pub use error::Error;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS), ts(export))]
#[serde(rename_all = "camelCase", rename = "ShipFitting")]
pub struct Fitting {
    pub layout: ShipLayout,

    pub slots_targeted: Vec<Targeted>,
    pub slots_untargeted: Vec<Untargeted>,
    pub slots_passive: Vec<Passive>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS), ts(export))]
#[serde(rename_all = "camelCase", rename = "ShipFittingInfrastructureUsage")]
pub struct InfrastructureUsage {
    pub cpu: u16,
    pub powergrid: u16,

    pub slots_passive: usize,
    pub slots_targeted: usize,
    pub slots_untargeted: usize,
}

impl Default for Fitting {
    fn default() -> Self {
        Self {
            layout: ShipLayout::default(),
            slots_targeted: vec![Targeted::RookieMiner, Targeted::RookieLaser],
            slots_untargeted: vec![Untargeted::RookieArmorRepair],
            slots_passive: vec![Passive::RookieArmorPlate],
        }
    }
}

impl From<ShipLayout> for Fitting {
    fn from(layout: ShipLayout) -> Self {
        Self {
            layout,
            slots_targeted: vec![],
            slots_untargeted: vec![],
            slots_passive: vec![],
        }
    }
}

impl Fitting {
    #[must_use]
    pub fn to_usage(&self, statics: &Statics) -> InfrastructureUsage {
        let mut cpu = 0;
        let mut powergrid = 0;
        for id in &self.slots_targeted {
            let m = statics.modules_targeted.get(id);
            cpu += m.required_cpu;
            powergrid += m.required_powergrid;
        }
        for id in &self.slots_untargeted {
            let m = statics.modules_untargeted.get(id);
            cpu += m.required_cpu;
            powergrid += m.required_powergrid;
        }
        for id in &self.slots_passive {
            let m = statics.modules_passive.get(id);
            cpu += m.required_cpu;
            powergrid += m.required_powergrid;
        }

        InfrastructureUsage {
            cpu,
            powergrid,
            slots_targeted: self.slots_targeted.len(),
            slots_untargeted: self.slots_untargeted.len(),
            slots_passive: self.slots_passive.len(),
        }
    }

    /// Check if the fitting is valid or not
    /// # Errors
    /// When Fitting isnt valid the Error states why
    pub fn is_valid(&self, statics: &Statics) -> Result<(), Error> {
        let usage = self.to_usage(statics);
        let layout = statics.ship_layouts.get(&self.layout);
        // More modules than layout offers
        if usage.slots_targeted > layout.slots_targeted.into() {
            return Err(Error::TooManyTargetedModules {
                wants: usage.slots_targeted,
                max: layout.slots_targeted,
            });
        }
        if usage.slots_untargeted > layout.slots_untargeted.into() {
            return Err(Error::TooManyUntargetedModules {
                wants: usage.slots_untargeted,
                max: layout.slots_untargeted,
            });
        }
        if usage.slots_passive > layout.slots_passive.into() {
            return Err(Error::TooManyPassiveModules {
                wants: usage.slots_passive,
                max: layout.slots_passive,
            });
        }

        // Check cpu / powergrid
        if usage.cpu > layout.cpu {
            return Err(Error::Cpu {
                wants: usage.cpu,
                max: layout.cpu,
            });
        }
        if usage.powergrid > layout.powergrid {
            return Err(Error::Powergrid {
                wants: usage.powergrid,
                max: layout.powergrid,
            });
        }

        // Dead on undock
        let collateral = self.maximum_collateral(statics);
        if !collateral.is_alive() {
            return Err(Error::StructureZero);
        }

        Ok(())
    }

    #[must_use]
    #[allow(clippy::cast_sign_loss)]
    pub fn maximum_collateral(&self, statics: &Statics) -> Collateral {
        #[allow(clippy::cast_sign_loss)]
        const fn add(base: u16, add: i16) -> u16 {
            if add >= 0 {
                base.saturating_add(add as u16)
            } else {
                let b = add.saturating_abs() as u16;
                base.saturating_sub(b)
            }
        }

        let mut collateral = statics.ship_layouts.get(&self.layout).collateral;

        for id in &self.slots_passive {
            let m = statics.modules_passive.get(id);
            collateral.armor = add(collateral.armor, m.hitpoints_armor);
        }

        collateral
    }
}

#[test]
fn default_fitting_is_valid() {
    let statics = crate::fixed::Statics::default();
    assert_eq!(Fitting::default().is_valid(&statics), Ok(()));
}

#[test]
#[allow(clippy::cast_sign_loss)]
fn collateral_without_modules_correct() {
    let statics = Statics::default();
    let expected = statics.ship_layouts.get(&ShipLayout::Abis);
    let fitting = Fitting {
        layout: ShipLayout::Abis,
        slots_targeted: vec![],
        slots_untargeted: vec![],
        slots_passive: vec![],
    };
    let result = fitting.maximum_collateral(&statics);
    assert_eq!(result, expected.collateral);
}

#[test]
#[allow(clippy::cast_sign_loss)]
fn collateral_of_default_fitting_correct() {
    let statics = Statics::default();
    let fitting = Fitting::default();
    let expected_layout = statics.ship_layouts.get(&fitting.layout);
    let expected_passive = statics.modules_passive.get(&fitting.slots_passive[0]);
    let result = fitting.maximum_collateral(&statics);
    assert_eq!(
        result,
        Collateral {
            capacitor: expected_layout.collateral.capacitor,
            armor: (expected_layout.collateral.armor) + (expected_passive.hitpoints_armor as u16),
            structure: expected_layout.collateral.structure,
        }
    );
}
