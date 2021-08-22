use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::fixed::facility::Service;
use crate::persist::site::Site;

// TODO: can become untagged with renaming the properties to a more speaking name

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", tag = "type", content = "args")]
pub enum SiteInstruction {
    ModuleUntargeted(ModuleUntargeted),
    ModuleTargeted(ModuleTargeted),
    SelfDestruct,
    Facility(Facility),
    Warp(Warp),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "SiteInstructionModuleUntargeted")]
pub struct ModuleUntargeted {
    pub module_index: u8,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "SiteInstructionModuleTargeted")]
pub struct ModuleTargeted {
    pub target_index_in_site: u8,
    pub module_index: u8,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "SiteInstructionFacility")]
pub struct Facility {
    pub target_index_in_site: u8,
    pub service: Service,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "SiteInstructionWarp")]
pub struct Warp {
    pub target: Site,
}

#[cfg(feature = "typescript")]
ts_rs::export! {
    SiteInstruction => "site-instruction.ts",
    ModuleUntargeted => "site-instruction-module-untargeted.ts",
    ModuleTargeted => "site-instruction-module-targeted.ts",
    Facility => "site-instruction-facility.ts",
    Warp => "site-instruction-warp.ts",
}

/// Filter instructions to be possible afterwards.
///
/// For example you can not do anything besides warping or docking.
/// Also its not possible to use the same module twice.
#[must_use]
pub fn filter_possible(instructions: &[SiteInstruction]) -> Vec<SiteInstruction> {
    let mut untargeted = HashMap::new();
    let mut targeted = HashMap::new();
    let mut standalone = None;

    for i in instructions.iter().copied() {
        match i {
            SiteInstruction::ModuleUntargeted(m) => {
                standalone = None;
                untargeted.insert(m.module_index, i);
            }
            SiteInstruction::ModuleTargeted(m) => {
                standalone = None;
                targeted.insert(m.module_index, i);
            }
            SiteInstruction::SelfDestruct
            | SiteInstruction::Facility(_)
            | SiteInstruction::Warp(_) => {
                untargeted.clear();
                targeted.clear();
                standalone = Some(i);
            }
        }
    }
    untargeted
        .values()
        .chain(targeted.values())
        .chain(standalone.iter())
        .copied()
        .collect()
}

#[test]
fn can_identify_module_untargeted() {
    let data = SiteInstruction::ModuleUntargeted(ModuleUntargeted { module_index: 4 });
    crate::test_helper::can_serde_parse(&data);
}

#[test]
fn can_identify_module_targeted() {
    let data = SiteInstruction::ModuleTargeted(ModuleTargeted {
        target_index_in_site: 42,
        module_index: 4,
    });
    crate::test_helper::can_serde_parse(&data);
}

#[test]
fn can_identify_self_destruct() {
    let data = SiteInstruction::SelfDestruct;
    crate::test_helper::can_serde_parse(&data);
}

#[test]
fn can_identify_warp() {
    let data = SiteInstruction::Warp(Warp {
        target: Site::Station(42),
    });
    crate::test_helper::can_serde_parse(&data);
}

#[test]
fn can_identify_facility() {
    let data = SiteInstruction::Facility(Facility {
        target_index_in_site: 42,
        service: Service::Dock,
    });
    crate::test_helper::can_serde_parse(&data);
}
