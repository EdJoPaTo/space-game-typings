use serde::{Deserialize, Serialize};

use crate::fixed::npc_faction::NpcFaction;
use crate::fixed::{facility, lifeless, shiplayout};
use crate::persist::player;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
// #[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum SiteEntity {
    Facility(Facility),
    Lifeless(Lifeless),
    Npc(Npc),
    Player(Player),
}

impl From<&crate::persist::site_entity::SiteEntity> for SiteEntity {
    fn from(entity: &crate::persist::site_entity::SiteEntity) -> Self {
        match entity {
            crate::persist::site_entity::SiteEntity::Facility(info) => {
                Self::Facility(Facility { id: info.id })
            }
            crate::persist::site_entity::SiteEntity::Lifeless(info) => Self::Lifeless(Lifeless {
                id: info.id.to_string(),
            }),
            crate::persist::site_entity::SiteEntity::Npc(info) => Self::Npc(Npc {
                faction: info.faction,
                shiplayout: info.ship.fitting.layout.to_string(),
            }),
            crate::persist::site_entity::SiteEntity::Player(info) => Self::Player(Player {
                id: info.id.to_string(),
                shiplayout: info.shiplayout.to_string(),
            }),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "SiteEntityFacility")]
pub struct Facility {
    pub id: facility::Identifier,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "SiteEntityLifeless")]
pub struct Lifeless {
    pub id: lifeless::Identifier,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "SiteEntityNpc")]
pub struct Npc {
    pub faction: NpcFaction,
    pub shiplayout: shiplayout::Identifier,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "SiteEntityPlayer")]
pub struct Player {
    pub id: player::Identifier,
    pub shiplayout: shiplayout::Identifier,
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
        id: facility::Identifier::Stargate,
    });
    crate::test_helper::can_serde_parse(&data);
}

#[test]
fn can_parse_lifeless() {
    let data = SiteEntity::Lifeless(Lifeless {
        id: "lifelessAsteroid".to_string(),
    });
    crate::test_helper::can_serde_parse(&data);
}

#[test]
fn can_parse_npc() {
    let data = SiteEntity::Npc(Npc {
        faction: NpcFaction::Pirates,
        shiplayout: "shiplayoutFrigate".to_string(),
    });
    crate::test_helper::can_serde_parse(&data);
}

#[test]
fn can_parse_player() {
    let data = SiteEntity::Player(Player {
        id: "player-tg-666".to_string(),
        shiplayout: "shiplayoutFrigate".to_string(),
    });
    crate::test_helper::can_serde_parse(&data);
}
