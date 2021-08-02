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
