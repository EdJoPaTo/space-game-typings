use serde::{Deserialize, Serialize};

use crate::fixed::facility::Service;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", tag = "type", content = "args")]
pub enum SiteInstruction {
    ModuleUntargeted(ModuleUntargeted),
    ModuleTargeted(ModuleTargeted),
    Facility(Facility),
    Warp(Warp),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "SiteInstructionModuleUntargeted")]
pub struct ModuleUntargeted {
    pub module_index: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "SiteInstructionModuleTargeted")]
pub struct ModuleTargeted {
    pub target_index_in_site: u8,
    pub module_index: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "SiteInstructionFacility")]
pub struct Facility {
    pub target_index_in_site: u8,
    pub service: Service,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "SiteInstructionWarp")]
pub struct Warp {
    pub site_unique: String,
}

#[cfg(test)]
ts_rs::export! {
    SiteInstruction => "site-instruction.ts",
    ModuleUntargeted => "site-instruction-module-untargeted.ts",
    ModuleTargeted => "site-instruction-module-targeted.ts",
    Facility => "site-instruction-facility.ts",
    Warp => "site-instruction-warp.ts",
}

#[test]
fn can_identify_warp() {
    let data = SiteInstruction::Warp(Warp {
        site_unique: "666".to_string(),
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
