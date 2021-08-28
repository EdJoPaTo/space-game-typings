use serde::{Deserialize, Serialize};

use crate::entity::Health;
use crate::fixed::item::Ore;
use crate::fixed::npc_faction::NpcFaction;
use crate::fixed::shiplayout::ShipLayout;
use crate::fixed::{facility, Statics};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", untagged)]
pub enum SiteEntity {
    Asteroid(Asteroid),
    Facility(Facility),
    Npc(Npc),
    Player(Player),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "SiteEntityAsteroid")]
pub struct Asteroid {
    pub ore: Ore,
    #[serde(flatten)]
    pub health: Health,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "SiteEntityFacility")]
pub struct Facility {
    pub facility: facility::Facility,
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
    pub player: crate::player::Player,
    pub shiplayout: ShipLayout,
    #[serde(flatten)]
    pub health: Health,
}

#[cfg(feature = "typescript")]
ts_rs::export! {
    Asteroid,
    Facility,
    Npc,
    Player,
    SiteEntity => "typescript/generated-site-entity.ts"
}

impl SiteEntity {
    #[must_use]
    pub fn from(statics: &Statics, entity: &crate::site::Entity) -> Self {
        match entity {
            crate::site::Entity::Asteroid(a) => Self::Asteroid(Asteroid {
                ore: a.ore,
                health: a.collateral.calc_health_raw(0, a.max_structure),
            }),
            crate::site::Entity::Facility(f) => Self::Facility(Facility { facility: *f }),
            crate::site::Entity::Npc((faction, ship)) => Self::Npc(Npc {
                faction: *faction,
                shiplayout: ship.fitting.layout,
                health: ship.to_health(statics),
            }),
            crate::site::Entity::Player((player, ship)) => Self::Player(Player {
                player: *player,
                shiplayout: ship.fitting.layout,
                health: ship.to_health(statics),
            }),
        }
    }
}

#[test]
fn can_parse_asteroid() {
    let data = SiteEntity::Asteroid(Asteroid {
        ore: Ore::Aromit,
        health: Health {
            armor: 0.0,
            structure: 42.0,
        },
    });
    crate::test_helper::can_serde_parse(&data);
}

#[test]
fn can_parse_facility() {
    let data = SiteEntity::Facility(Facility {
        facility: facility::Facility::Stargate,
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
        player: crate::player::Player::Telegram(666),
        shiplayout: ShipLayout::default(),
        health: Health {
            armor: 0.0,
            structure: 42.0,
        },
    });
    crate::test_helper::can_serde_parse(&data);
}
