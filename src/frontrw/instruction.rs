use serde::{Deserialize, Serialize};

use crate::fixed::facility::Service;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
// #[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", tag = "step")]
pub enum Instruction {
    Untargeted(Untargeted),
    Targeted(Targeted),
    Movement(Movement),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
// #[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum Untargeted {
    Module(ModuleUntargeted),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
// #[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum Targeted {
    Facility(Facility),
    Module(ModuleTargeted),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
// #[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum Movement {
    Undock(Undock),
    Warp(Warp),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "InstructionUntargetedModule")]
pub struct ModuleUntargeted {
    pub module_index: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "InstructionTargetedModule")]
pub struct ModuleTargeted {
    pub target_index_in_site: u8,
    pub module_index: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "InstructionTargetedFacility")]
pub struct Facility {
    pub target_index_in_site: u8,
    pub service: Service,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "InstructionMovementWarp")]
pub struct Warp {
    pub site_unique: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "InstructionMovementUndock")]
pub struct Undock {
    ship_id: u8,
}

#[cfg(test)]
ts_rs::export! {
    ModuleUntargeted => "instruction-untargeted-module.ts",
    ModuleTargeted => "instruction-targeted-module.ts",
    Facility => "instruction-targeted-facility.ts",
    Warp => "instruction-movement-warp.ts",
    Undock => "instruction-movement-undock.ts",
}

#[test]
fn can_identify_undock() {
    let data = Instruction::Movement(Movement::Undock(Undock { ship_id: 42 }));
    crate::test_helper::can_serde_parse(&data);
}

#[test]
fn can_identify_warp() {
    let data = Instruction::Movement(Movement::Warp(Warp {
        site_unique: "666".to_string(),
    }));
    crate::test_helper::can_serde_parse(&data);
}

#[test]
fn can_identify_facility() {
    let data = Instruction::Targeted(Targeted::Facility(Facility {
        target_index_in_site: 42,
        service: Service::Dock,
    }));
    crate::test_helper::can_serde_parse(&data);
}

#[test]
fn can_identify_module_untargeted() {
    let data = Instruction::Untargeted(Untargeted::Module(ModuleUntargeted { module_index: 4 }));
    crate::test_helper::can_serde_parse(&data);
}

#[test]
fn can_identify_module_targeted() {
    let data = Instruction::Targeted(Targeted::Module(ModuleTargeted {
        target_index_in_site: 42,
        module_index: 4,
    }));
    crate::test_helper::can_serde_parse(&data);
}
