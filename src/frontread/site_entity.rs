use serde::{Deserialize, Serialize};

use crate::fixed::npc_faction::NpcFaction;
use crate::fixed::shiplayout::ShipLayout;
use crate::fixed::{facility, lifeless, LifelessThingies, Statics};
use crate::persist::player;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
// #[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum SiteEntity {
    Facility(Facility),
    Lifeless(Lifeless),
    Npc(Npc),
    Player(Player),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "SiteEntityFacility")]
pub struct Facility {
    pub id: facility::Facility,
}

impl From<&crate::persist::site_entity::Facility> for Facility {
    fn from(info: &crate::persist::site_entity::Facility) -> Self {
        Self { id: info.id }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "SiteEntityLifeless")]
pub struct Lifeless {
    pub id: lifeless::Lifeless,
    pub armor: f32,
    pub structure: f32,
}

impl Lifeless {
    #[must_use]
    pub fn new(statics: &LifelessThingies, info: &crate::persist::site_entity::Lifeless) -> Self {
        let (armor, structure) = statics
            .get(&info.id)
            .map(|shelf| {
                info.status
                    .health_percentage((shelf.hitpoints_armor, shelf.hitpoints_structure))
            })
            .unwrap_or_default();
        Self {
            id: info.id,
            armor,
            structure,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "SiteEntityNpc")]
pub struct Npc {
    pub faction: NpcFaction,
    pub shiplayout: ShipLayout,
    pub armor: f32,
    pub structure: f32,
}

impl Npc {
    #[must_use]
    pub fn new(statics: &Statics, info: &crate::persist::site_entity::Npc) -> Self {
        let (armor, structure) = info
            .status
            .health_percentage_layout(statics, &info.fitting)
            .unwrap_or_default();
        Self {
            faction: info.faction,
            shiplayout: info.fitting.layout,
            armor,
            structure,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "SiteEntityPlayer")]
pub struct Player {
    pub id: player::Identifier,
    pub shiplayout: ShipLayout,
    pub armor: f32,
    pub structure: f32,
}

impl Player {
    #[must_use]
    pub fn new(
        statics: &Statics,
        info: &crate::persist::site_entity::Player,
        ship: &crate::persist::ship::Ship,
    ) -> Self {
        let (armor, structure) = ship
            .status
            .health_percentage_layout(statics, &ship.fitting)
            .unwrap_or_default();
        Self {
            id: info.id.to_string(),
            shiplayout: ship.fitting.layout,
            armor,
            structure,
        }
    }
}

#[cfg(test)]
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
        armor: 0.0,
        structure: 42.0,
    });
    crate::test_helper::can_serde_parse(&data);
}

#[test]
fn can_parse_npc() {
    let data = SiteEntity::Npc(Npc {
        faction: NpcFaction::Pirates,
        shiplayout: ShipLayout::RookieShip,
        armor: 0.0,
        structure: 42.0,
    });
    crate::test_helper::can_serde_parse(&data);
}

#[test]
fn can_parse_player() {
    let data = SiteEntity::Player(Player {
        id: "player-tg-666".to_string(),
        shiplayout: ShipLayout::RookieShip,
        armor: 0.0,
        structure: 42.0,
    });
    crate::test_helper::can_serde_parse(&data);
}
