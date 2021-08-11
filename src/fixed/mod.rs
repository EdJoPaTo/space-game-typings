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

pub type Facilites = HashMap<facility::Facility, facility::Details>;
pub type LifelessThingies = HashMap<lifeless::Lifeless, lifeless::Details>;
pub type ModulesPassive = HashMap<module::PassiveIdentifier, module::Passive>;
pub type ModulesUntargeted = HashMap<module::UntargetedIdentifier, module::Untargeted>;
pub type ModulesTargeted = HashMap<module::TargetedIdentifier, module::Targeted>;
pub type ShipLayouts = HashMap<shiplayout::ShipLayout, shiplayout::Details>;
pub type Solarsystems = HashMap<solarsystem::Solarsystem, solarsystem::Details>;

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

impl Default for Statics {
    fn default() -> Self {
        Self {
            facilities: parse(include_str!("../../static/facility.yaml")),
            lifeless: parse(include_str!("../../static/lifeless.yaml")),
            modules_passive: parse(include_str!("../../static/module-passive.yaml")),
            modules_untargeted: parse(include_str!("../../static/module-untargeted.yaml")),
            modules_targeted: parse(include_str!("../../static/module-targeted.yaml")),
            ship_layouts: parse(include_str!("../../static/ship-layout.yaml")),
            solarsystems: parse(include_str!("../../static/solarsystem.yaml")),
        }
    }
}

fn parse<T>(yaml_str: &str) -> T
where
    T: serde::de::DeserializeOwned,
{
    serde_yaml::from_str::<T>(yaml_str).expect("failed to parse statics")
}

#[test]
fn can_generate_statics_from_include() {
    Statics::default();
}
