use serde::{Deserialize, Serialize};

use crate::entity::Health;
use crate::fixed::npc_faction::NpcFaction;
use crate::fixed::shiplayout::ShipLayout;
use crate::fixed::{facility, lifeless, Statics};

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

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "SiteEntityLifeless")]
pub struct Lifeless {
    pub id: lifeless::Lifeless,
    #[serde(flatten)]
    pub health: Health,
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

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "SiteEntityPlayer")]
pub struct Player {
    pub id: crate::player::Player,
    pub shiplayout: ShipLayout,
    #[serde(flatten)]
    pub health: Health,
}

#[cfg(feature = "typescript")]
ts_rs::export! {
    // SiteEntity => "site-entity.ts",
    Facility => "site-entity-facility.ts",
    Lifeless => "site-entity-lifeless.ts",
    Npc => "site-entity-npc.ts",
    Player => "site-entity-player.ts",
}

impl SiteEntity {
    #[must_use]
    pub fn from(statics: &Statics, entity: &crate::site::Entity) -> Self {
        match entity {
            crate::site::Entity::Facility(f) => Self::Facility(Facility { id: *f }),
            crate::site::Entity::Lifeless(l) => Self::Lifeless(Lifeless {
                id: l.id,
                health: l
                    .collateral
                    .calc_health(statics.lifeless.get(&l.id).collateral),
            }),
            crate::site::Entity::Npc((faction, ship)) => Self::Npc(Npc {
                faction: *faction,
                shiplayout: ship.fitting.layout,
                health: ship.to_health(statics),
            }),
            crate::site::Entity::Player((player, ship)) => Self::Player(Player {
                id: *player,
                shiplayout: ship.fitting.layout,
                health: ship.to_health(statics),
            }),
        }
    }
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
        id: crate::player::Player::Telegram(666),
        shiplayout: ShipLayout::default(),
        health: Health {
            armor: 0.0,
            structure: 42.0,
        },
    });
    crate::test_helper::can_serde_parse(&data);
}
