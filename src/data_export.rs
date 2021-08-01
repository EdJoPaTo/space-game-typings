use std::collections::HashMap;

use super::fixed::facility::{self, Facility};
use super::fixed::lifeless::Lifeless;
use super::fixed::module;
use super::fixed::shiplayout::ShipLayout;
use super::fixed::solarsystem::Solarsystem;

pub type Facilites = HashMap<facility::Identifier, Facility>;
pub type LifelessThingies = HashMap<String, Lifeless>;
pub type ModulesPassive = HashMap<String, module::Passive>;
pub type ModulesUntargeted = HashMap<String, module::Untargeted>;
pub type ModulesTargeted = HashMap<String, module::Targeted>;
pub type ShipLayouts = HashMap<String, ShipLayout>;
pub type Solarsystems = HashMap<String, Solarsystem>;
