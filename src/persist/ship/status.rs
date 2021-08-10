use serde::{Deserialize, Serialize};

use crate::fixed::shiplayout::ShipQuality;
use crate::fixed::Statics;

use super::Fitting;

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
    Status => "ship-status.ts",
}

impl Status {
    #[must_use]
    #[allow(clippy::cast_sign_loss)]
    pub fn new(statics: &Statics, fitting: &Fitting) -> Option<Self> {
        let mut capacitor: i16 = 0;
        let mut armor: i16 = 0;
        let mut structure = 0;

        let layout = statics.ship_layouts.get(&fitting.layout)?;
        let qualities = fitting
            .slots_passive
            .iter()
            .filter_map(|o| statics.modules_passive.get(o))
            .flat_map(|o| &o.qualities)
            .chain(&layout.qualities);
        for (q, amount) in qualities {
            match q {
                ShipQuality::HitpointsArmor => {
                    armor = amount.saturating_add(armor);
                }
                ShipQuality::HitpointsStructure => {
                    structure = amount.saturating_add(structure);
                }
                ShipQuality::Capacitor => {
                    capacitor = amount.saturating_add(capacitor);
                }
                ShipQuality::CapacitorRecharge => {}
            }
        }

        Some(Status {
            capacitor: capacitor.saturating_abs() as u16,
            hitpoints_armor: armor.saturating_abs() as u16,
            hitpoints_structure: structure.saturating_abs() as u16,
        })
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

    #[must_use]
    /// (Armor, Structure) within 0.0..=1.0
    pub fn health_percentage(self, max: (u16, u16)) -> (f32, f32) {
        let armor = f32::from(self.hitpoints_armor) / f32::from(max.0);
        let structure = f32::from(self.hitpoints_structure) / f32::from(max.1);
        (armor, structure)
    }

    #[must_use]
    /// (Armor, Structure) within 0.0..=1.0
    pub fn health_percentage_layout(
        self,
        statics: &Statics,
        fitting: &Fitting,
    ) -> Option<(f32, f32)> {
        let max = Self::new(statics, fitting)?;
        Some(self.health_percentage((max.hitpoints_armor, max.hitpoints_structure)))
    }
}

#[test]
#[allow(clippy::cast_sign_loss)]
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
            capacitor: *expected.qualities.get(&ShipQuality::Capacitor).unwrap() as u16,
            hitpoints_armor: *expected
                .qualities
                .get(&ShipQuality::HitpointsArmor)
                .unwrap() as u16,
            hitpoints_structure: *expected
                .qualities
                .get(&ShipQuality::HitpointsStructure)
                .unwrap() as u16,
        })
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
    let result = Status::new(&statics, &fitting);
    assert_eq!(
        result,
        Some(Status {
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
        })
    );
}
