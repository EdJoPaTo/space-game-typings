use std::collections::HashMap;

pub mod facility;
pub mod lifeless;
pub mod module;
pub mod npc_faction;
pub mod shiplayout;
pub mod site;
pub mod solarsystem;

#[cfg(test)]
mod do_data_export;

pub type Facilites = HashMap<facility::Identifier, facility::Facility>;
pub type LifelessThingies = HashMap<lifeless::Identifier, lifeless::Lifeless>;
pub type ModulesPassive = HashMap<module::PassiveIdentifier, module::Passive>;
pub type ModulesUntargeted = HashMap<module::UntargetedIdentifier, module::Untargeted>;
pub type ModulesTargeted = HashMap<module::TargetedIdentifier, module::Targeted>;
pub type ShipLayouts = HashMap<shiplayout::Identifier, shiplayout::ShipLayout>;
pub type Solarsystems = HashMap<solarsystem::Identifier, solarsystem::Solarsystem>;

#[derive(Debug)]
pub struct Statics {
    pub facilities: Facilites,
    pub lifeless: LifelessThingies,
    pub modules_passive: ModulesPassive,
    pub modules_untargeted: ModulesUntargeted,
    pub modules_targeted: ModulesTargeted,
    pub ship_layouts: ShipLayouts,
    pub solarsystems: Solarsystems,
}

impl Statics {
    pub fn import_yaml(basepath: &str) -> anyhow::Result<Self> {
        Ok(Self {
            facilities: import_yaml(basepath, "facility")?,
            lifeless: import_yaml(basepath, "lifeless")?,
            modules_passive: import_yaml(basepath, "module-passive")?,
            modules_untargeted: import_yaml(basepath, "module-untargeted")?,
            modules_targeted: import_yaml(basepath, "module-targeted")?,
            ship_layouts: import_yaml(basepath, "ship-layout")?,
            solarsystems: import_yaml(basepath, "solarsystem")?,
        })
    }
}

fn import_yaml<T>(basepath: &str, filename: &str) -> anyhow::Result<T>
where
    T: serde::de::DeserializeOwned,
{
    let yaml_str = std::fs::read_to_string(&format!("{}/{}.yaml", basepath, filename))?;
    let value = serde_yaml::from_str::<T>(&yaml_str)?;
    Ok(value)
}

#[test]
fn can_import_yaml() {
    let result = Statics::import_yaml("static");
    println!("{:?}", result);
    assert!(result.is_ok());
}
