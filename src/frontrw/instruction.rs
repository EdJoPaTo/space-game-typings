use serde::{Deserialize, Serialize};

use crate::fixed::facility::Service;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
// #[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum Instruction {
    ModuleUntargeted(ModuleUntargeted),
    ModuleTargeted(ModuleTargeted),
    Facility(Facility),
    Undock,
    Warp(Warp),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "InstructionModuleUntargeted")]
pub struct ModuleUntargeted {
    pub module_index: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "InstructionModuleTargeted")]
pub struct ModuleTargeted {
    pub target_index_in_site: u8,
    pub module_index: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "InstructionFacility")]
pub struct Facility {
    pub target_index_in_site: u8,
    pub service: Service,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "InstructionWarp")]
pub struct Warp {
    pub site_unique: String,
}

#[cfg(test)]
ts_rs::export! {
    ModuleUntargeted => "instruction-module-untargeted.ts",
    ModuleTargeted => "instruction-module-targeted.ts",
    Facility => "instruction-facility.ts",
    Warp => "instruction-warp.ts",
}

#[test]
fn can_identify_undock() {
    let data = Instruction::Undock;
    crate::test_helper::can_serde_parse(&data);
}

#[test]
fn can_identify_warp() {
    let data = Instruction::Warp(Warp {
        site_unique: "666".to_string(),
    });
    crate::test_helper::can_serde_parse(&data);
}

#[test]
fn can_identify_facility() {
    let data = Instruction::Facility(Facility {
        target_index_in_site: 42,
        service: Service::Dock,
    });
    crate::test_helper::can_serde_parse(&data);
}

#[test]
fn can_identify_module_untargeted() {
    let data = Instruction::ModuleUntargeted(ModuleUntargeted { module_index: 4 });
    crate::test_helper::can_serde_parse(&data);
}

#[test]
fn can_identify_module_targeted() {
    let data = Instruction::ModuleTargeted(ModuleTargeted {
        target_index_in_site: 42,
        module_index: 4,
    });
    crate::test_helper::can_serde_parse(&data);
}
