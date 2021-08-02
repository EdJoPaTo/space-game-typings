use serde::{Deserialize, Serialize};

use crate::fixed::facility::Service;

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", tag = "type", content = "info")]
pub enum Instruction {
    Undock(Undock),
    Warp(Warp),
    UseFacility(UseFacility),
    ModuleTargeted(ModuleTargeted),
    ModuleUntargeted(ModuleUntargeted),
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "InstructionModuleUntargeted")]
pub struct ModuleUntargeted {
    pub module_index: u8,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "InstructionModuleTargeted")]
pub struct ModuleTargeted {
    pub target_index_in_site: u8,
    pub module_index: u8,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "InstructionUseFacility")]
pub struct UseFacility {
    pub target_index_in_site: u8,
    pub service: Service,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "InstructionWarp")]
pub struct Warp {
    pub site_unique: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "InstructionUndock")]
pub struct Undock {
    ship_id: u8,
}

#[cfg(test)]
ts_rs::export! {
    Instruction => "instruction.ts",
    ModuleUntargeted => "instruction-module-untargeted.ts",
    ModuleTargeted => "instruction-module-targeted.ts",
    UseFacility => "instruction-use-facility.ts",
    Warp => "instruction-warp.ts",
    Undock => "instruction-undock.ts",
}

#[test]
fn can_identify_undock() -> anyhow::Result<()> {
    let data = Instruction::Undock(Undock { ship_id: 42 });
    let json = serde_json::to_string_pretty(&data)?;
    println!("json {}", json);
    let some = serde_json::from_str::<Instruction>(&json)?;
    if let Instruction::Undock(_) = some {
        Ok(())
    } else {
        panic!("wrong!");
    }
}

#[test]
fn can_identify_warp() -> anyhow::Result<()> {
    let data = Instruction::Warp(Warp {
        site_unique: "666".to_string(),
    });
    let json = serde_json::to_string_pretty(&data)?;
    println!("json {}", json);
    let some = serde_json::from_str::<Instruction>(&json)?;
    if let Instruction::Warp(_) = some {
        Ok(())
    } else {
        panic!("wrong!");
    }
}

#[test]
fn can_identify_facility() -> anyhow::Result<()> {
    let data = Instruction::UseFacility(UseFacility {
        target_index_in_site: 42,
        service: Service::Dock,
    });
    let json = serde_json::to_string_pretty(&data)?;
    println!("json {}", json);
    let some = serde_json::from_str::<Instruction>(&json)?;
    if let Instruction::UseFacility(_) = some {
        Ok(())
    } else {
        panic!("wrong!");
    }
}

#[test]
fn can_identify_module_untargeted() -> anyhow::Result<()> {
    let data = Instruction::ModuleUntargeted(ModuleUntargeted { module_index: 4 });
    let json = serde_json::to_string_pretty(&data)?;
    println!("json {}", json);
    let some = serde_json::from_str::<Instruction>(&json)?;
    if let Instruction::ModuleUntargeted(_) = some {
        Ok(())
    } else {
        panic!("wrong!");
    }
}

#[test]
fn can_identify_module_targeted() -> anyhow::Result<()> {
    let data = Instruction::ModuleTargeted(ModuleTargeted {
        target_index_in_site: 42,
        module_index: 4,
    });
    let json = serde_json::to_string_pretty(&data)?;
    println!("json {}", json);
    let some = serde_json::from_str::<Instruction>(&json)?;
    if let Instruction::ModuleTargeted(_) = some {
        Ok(())
    } else {
        panic!("wrong!");
    }
}
