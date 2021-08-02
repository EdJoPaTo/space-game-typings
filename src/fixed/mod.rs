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
pub type LifelessThingies = HashMap<String, lifeless::Lifeless>;
pub type ModulesPassive = HashMap<String, module::Passive>;
pub type ModulesUntargeted = HashMap<String, module::Untargeted>;
pub type ModulesTargeted = HashMap<String, module::Targeted>;
pub type ShipLayouts = HashMap<String, shiplayout::ShipLayout>;
pub type Solarsystems = HashMap<String, solarsystem::Solarsystem>;
