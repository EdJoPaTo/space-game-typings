use self::database::Database;

pub mod facility;
pub mod lifeless;
pub mod module;
pub mod npc_faction;
pub mod round_effect;
pub mod shiplayout;
pub mod solarsystem;

mod database;

#[cfg(test)]
mod do_data_export;

pub type Facilites = Database<facility::Facility, facility::Details>;
pub type LifelessThingies = Database<lifeless::Lifeless, lifeless::Details>;
pub type ModulesPassive = Database<module::passive::Passive, module::passive::Details>;
pub type ModulesUntargeted = Database<module::untargeted::Untargeted, module::untargeted::Details>;
pub type ModulesTargeted = Database<module::targeted::Targeted, module::targeted::Details>;
pub type ShipLayouts = Database<shiplayout::ShipLayout, shiplayout::Details>;
pub type Solarsystems = Database<solarsystem::Solarsystem, solarsystem::Details>;

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
            facilities: Database::p(include_str!("../../static/facility.yaml")),
            lifeless: Database::p(include_str!("../../static/lifeless.yaml")),
            modules_passive: Database::p(include_str!("../../static/module-passive.yaml")),
            modules_untargeted: Database::p(include_str!("../../static/module-untargeted.yaml")),
            modules_targeted: Database::p(include_str!("../../static/module-targeted.yaml")),
            ship_layouts: Database::p(include_str!("../../static/ship-layout.yaml")),
            solarsystems: Database::p(include_str!("../../static/solarsystem.yaml")),
        }
    }
}

#[test]
fn can_generate_default_statics() {
    Statics::default();
}
