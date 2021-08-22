use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::fixed::facility::Service;

use super::Site;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[serde(
    rename_all = "camelCase",
    rename = "SiteInstruction",
    tag = "type",
    content = "args"
)]
pub enum Instruction {
    ModuleUntargeted(UseModuleUntargeted),
    ModuleTargeted(UseModuleTargeted),
    SelfDestruct,
    Facility(UseFacilityService),
    Warp(Warp),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "SiteInstructionModuleUntargeted")]
pub struct UseModuleUntargeted {
    pub module_index: u8,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "SiteInstructionModuleTargeted")]
pub struct UseModuleTargeted {
    pub target_index_in_site: u8,
    pub module_index: u8,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "SiteInstructionFacility")]
pub struct UseFacilityService {
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
    Instruction => "site-instruction.ts",
    UseModuleUntargeted => "site-instruction-module-untargeted.ts",
    UseModuleTargeted => "site-instruction-module-targeted.ts",
    UseFacilityService => "site-instruction-facility.ts",
    Warp => "site-instruction-warp.ts",
}

/// Filter instructions to be possible afterwards.
///
/// For example you can not do anything besides warping or docking.
/// Also its not possible to use the same module twice.
#[must_use]
pub fn filter_possible(instructions: &[Instruction]) -> Vec<Instruction> {
    let mut untargeted = HashMap::new();
    let mut targeted = HashMap::new();
    let mut standalone = None;

    for i in instructions.iter().copied() {
        match i {
            Instruction::ModuleUntargeted(m) => {
                standalone = None;
                untargeted.insert(m.module_index, i);
            }
            Instruction::ModuleTargeted(m) => {
                standalone = None;
                targeted.insert(m.module_index, i);
            }
            Instruction::SelfDestruct | Instruction::Facility(_) | Instruction::Warp(_) => {
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

/// Flattens the instructions and returns them sorted
#[must_use]
pub fn sort<T>(instructions: &HashMap<T, Vec<Instruction>>) -> Vec<(T, Instruction)>
where
    T: Copy,
{
    let mut result: Vec<(T, Instruction)> = Vec::new();
    for (entity, instructions) in instructions {
        for instruction in filter_possible(instructions) {
            result.push((*entity, instruction));
        }
    }
    result.sort_by(|a, b| a.1.cmp(&b.1));
    result
}

#[test]
fn can_identify_module_untargeted() {
    let data = Instruction::ModuleUntargeted(UseModuleUntargeted { module_index: 4 });
    crate::test_helper::can_serde_parse(&data);
}

#[test]
fn can_identify_module_targeted() {
    let data = Instruction::ModuleTargeted(UseModuleTargeted {
        target_index_in_site: 42,
        module_index: 4,
    });
    crate::test_helper::can_serde_parse(&data);
}

#[test]
fn can_identify_self_destruct() {
    let data = Instruction::SelfDestruct;
    crate::test_helper::can_serde_parse(&data);
}

#[test]
fn can_identify_facility() {
    let data = Instruction::Facility(UseFacilityService {
        target_index_in_site: 42,
        service: Service::Dock,
    });
    crate::test_helper::can_serde_parse(&data);
}

#[test]
fn can_identify_warp() {
    let data = Instruction::Warp(Warp {
        target: Site::Station(42),
    });
    crate::test_helper::can_serde_parse(&data);
}

#[test]
fn sort_works() {
    let mut example = HashMap::new();
    example.insert(
        1,
        vec![Instruction::Warp(Warp {
            target: Site::Station(42),
        })],
    );
    example.insert(
        2,
        vec![
            Instruction::ModuleTargeted(UseModuleTargeted {
                module_index: 0,
                target_index_in_site: 0,
            }),
            Instruction::ModuleUntargeted(UseModuleUntargeted { module_index: 0 }),
        ],
    );
    let sorted = sort(&example);
    assert_eq!(sorted.len(), 3);
    assert_eq!(
        sorted[0],
        (
            2,
            Instruction::ModuleUntargeted(UseModuleUntargeted { module_index: 0 })
        )
    );
    assert_eq!(
        sorted[1],
        (
            2,
            Instruction::ModuleTargeted(UseModuleTargeted {
                module_index: 0,
                target_index_in_site: 0,
            })
        )
    );
    assert_eq!(
        sorted[2],
        (
            1,
            Instruction::Warp(Warp {
                target: Site::Station(42),
            })
        )
    );
}
