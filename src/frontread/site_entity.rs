use serde::{Deserialize, Serialize};

use crate::fixed::npc_faction::NpcFaction;
use crate::fixed::{facility, lifeless, shiplayout};
use crate::persist::player;

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "SiteEntityFacility")]
pub struct Facility {
    pub id: facility::Identifier,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "SiteEntityLifeless")]
pub struct Lifeless {
    pub id: lifeless::Identifier,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "SiteEntityNpc")]
pub struct Npc {
    pub faction: NpcFaction,
    pub shiplayout: shiplayout::Identifier,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "SiteEntityPlayer")]
pub struct Player {
    pub id: player::Identifer,
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
fn can_parse_entity_lifeless() -> anyhow::Result<()> {
    let origin = SiteEntity::Lifeless(Lifeless {
        id: "lifelessAsteroid".to_string(),
    });
    let json = serde_json::to_string_pretty(&origin)?;
    println!("json {}", json);

    let some = serde_json::from_str::<SiteEntity>(&json)?;
    println!("some {:?}", some);

    if let SiteEntity::Lifeless(info) = some {
        assert_eq!(info.id, "lifelessAsteroid");
        Ok(())
    } else {
        panic!();
    }
}

#[test]
fn can_parse_entity_npc() -> anyhow::Result<()> {
    let origin = SiteEntity::Npc(Npc {
        faction: NpcFaction::Pirates,
        shiplayout: "shiplayoutFrigate".to_string(),
    });
    let json = serde_json::to_string_pretty(&origin)?;
    println!("json {}", json);

    let some = serde_json::from_str::<SiteEntity>(&json)?;
    println!("some {:?}", some);

    if let SiteEntity::Npc(info) = some {
        assert_eq!(info.shiplayout, "shiplayoutFrigate");
        if let NpcFaction::Pirates = info.faction {
            Ok(())
        } else {
            panic!();
        }
    } else {
        panic!();
    }
}
