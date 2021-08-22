use serde::{Deserialize, Serialize};

use crate::fixed::npc_faction::NpcFaction;
use crate::fixed::shiplayout::ShipLayout;
use crate::fixed::{facility, lifeless, LifelessThingies, Statics};
use crate::ship::Ship;

use super::health::Health;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
// #[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum SiteEntity {
    Facility(Facility),
    Lifeless(Lifeless),
    Npc(Npc),
    Player(Player),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "SiteEntityFacility")]
pub struct Facility {
    pub id: facility::Facility,
}

impl From<&crate::fixed::facility::Facility> for Facility {
    fn from(info: &crate::fixed::facility::Facility) -> Self {
        Self { id: *info }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "SiteEntityLifeless")]
pub struct Lifeless {
    pub id: lifeless::Lifeless,
    #[serde(flatten)]
    pub health: Health,
}

impl Lifeless {
    #[must_use]
    pub fn new(statics: &LifelessThingies, info: &crate::persist::site_entity::Lifeless) -> Self {
        let shelf = statics.get(&info.id);
        let health = Health::from_raw(
            info.status,
            shelf.hitpoints_armor,
            shelf.hitpoints_structure,
        );
        Self {
            id: info.id,
            health,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "SiteEntityNpc")]
pub struct Npc {
    pub faction: NpcFaction,
    pub shiplayout: ShipLayout,
    #[serde(flatten)]
    pub health: Health,
}

impl Npc {
    #[must_use]
    pub fn new(statics: &Statics, info: &crate::persist::site_entity::Npc) -> Self {
        Self {
            faction: info.faction,
            shiplayout: info.ship.fitting.layout,
            health: Health::from_ship(statics, &info.ship),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "SiteEntityPlayer")]
pub struct Player {
    pub id: crate::persist::player::Player,
    pub shiplayout: ShipLayout,
    #[serde(flatten)]
    pub health: Health,
}

impl Player {
    #[must_use]
    pub fn new(statics: &Statics, id: crate::persist::player::Player, ship: &Ship) -> Self {
        Self {
            id,
            shiplayout: ship.fitting.layout,
            health: Health::from_ship(statics, ship),
        }
    }
}

#[cfg(feature = "typescript")]
ts_rs::export! {
    // SiteEntity => "site-entity.ts",
    Facility => "site-entity-facility.ts",
    Lifeless => "site-entity-lifeless.ts",
    Npc => "site-entity-npc.ts",
    Player => "site-entity-player.ts",
}

#[test]
fn can_parse_facility() {
    let data = SiteEntity::Facility(Facility {
        id: facility::Facility::Stargate,
    });
    crate::test_helper::can_serde_parse(&data);
}

#[test]
fn can_parse_lifeless() {
    let data = SiteEntity::Lifeless(Lifeless {
        id: lifeless::Lifeless::Asteroid,
        health: Health {
            armor: 0.0,
            structure: 42.0,
        },
    });
    crate::test_helper::can_serde_parse(&data);
}

#[test]
fn can_parse_npc() {
    let data = SiteEntity::Npc(Npc {
        faction: NpcFaction::Pirates,
        shiplayout: ShipLayout::default(),
        health: Health {
            armor: 0.0,
            structure: 42.0,
        },
    });
    crate::test_helper::can_serde_parse(&data);
}

#[test]
fn can_parse_player() {
    let data = SiteEntity::Player(Player {
        id: crate::persist::player::Player::Telegram(666),
        shiplayout: ShipLayout::default(),
        health: Health {
            armor: 0.0,
            structure: 42.0,
        },
    });
    crate::test_helper::can_serde_parse(&data);
}
