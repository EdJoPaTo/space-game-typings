use serde::{Deserialize, Serialize};

use crate::fixed::npc_faction::NpcFaction;
use crate::fixed::{facility, lifeless, shiplayout};
use crate::persist::player;

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", tag = "type", content = "info")]
pub enum SiteEntity {
    Facility(facility::Identifier),
    Lifeless(lifeless::Identifier),
    Npc(Npc),
    Player(Player),
}

impl From<crate::persist::site_entity::SiteEntity> for SiteEntity {
    fn from(entity: crate::persist::site_entity::SiteEntity) -> Self {
        match entity {
            crate::persist::site_entity::SiteEntity::Facility(info) => Self::Facility(info.id),
            crate::persist::site_entity::SiteEntity::Lifeless(info) => Self::Lifeless(info.id),
            crate::persist::site_entity::SiteEntity::Npc(info) => Self::Npc(Npc {
                faction: info.faction,
                shiplayout: info.shiplayout,
            }),
            crate::persist::site_entity::SiteEntity::Player(info) => Self::Player(Player {
                id: info.id,
                shiplayout: info.shiplayout,
            }),
        }
    }
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", rename = "SiteInners")]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct Inners {
    pub entities: Vec<SiteEntity>,
}
#[cfg(test)]
ts_rs::export! {
    SiteEntity => "site-entity.ts",
    Npc => "site-entity-npc.ts",
    Player => "site-entity-player.ts",
    Inners => "site-inners.ts",
}

#[test]
fn can_parse_entity_lifeless() -> anyhow::Result<()> {
    let origin = SiteEntity::Lifeless("lifelessAsteroid".to_string());
    let json = serde_json::to_string_pretty(&origin)?;
    println!("json {}", json);

    let some = serde_json::from_str::<SiteEntity>(&json)?;
    println!("some {:?}", some);

    if let SiteEntity::Lifeless(id) = some {
        assert_eq!(id, "lifelessAsteroid");
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
