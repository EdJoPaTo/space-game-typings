use self::database::Database;

pub mod facility;
pub mod item;
pub mod module;
pub mod npc_faction;
pub mod round_effect;
pub mod shiplayout;
pub mod solarsystem;

mod database;

#[cfg(test)]
mod do_data_export;

pub type Facilites = Database<facility::Facility, facility::Details>;
pub type Items = Database<item::Item, item::Details>;
pub type ModulesPassive = Database<module::Passive, module::PassiveDetails>;
pub type ModulesTargeted = Database<module::Targeted, module::TargetedDetails>;
pub type ModulesUntargeted = Database<module::Untargeted, module::UntargetedDetails>;
pub type ShipLayouts = Database<shiplayout::ShipLayout, shiplayout::Details>;
pub type Solarsystems = Database<solarsystem::Solarsystem, solarsystem::Details>;

pub struct Statics {
    pub facilities: Facilites,
    pub items: Items,
    pub modules_passive: ModulesPassive,
    pub modules_targeted: ModulesTargeted,
    pub modules_untargeted: ModulesUntargeted,
    pub ship_layouts: ShipLayouts,
    pub solarsystems: Solarsystems,
}

impl Default for Statics {
    fn default() -> Self {
        Self {
            facilities: Database::p(include_str!("../../static/facility.yaml")),
            items: Database::p(include_str!("../../static/item.yaml")),
            modules_passive: Database::p(include_str!("../../static/module-passive.yaml")),
            modules_targeted: Database::p(include_str!("../../static/module-targeted.yaml")),
            modules_untargeted: Database::p(include_str!("../../static/module-untargeted.yaml")),
            ship_layouts: Database::p(include_str!("../../static/ship-layout.yaml")),
            solarsystems: Database::p(include_str!("../../static/solarsystem.yaml")),
        }
    }
}

#[test]
fn can_generate_default_statics() {
    Statics::default();
}
