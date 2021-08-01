use serde::{Deserialize, Serialize};

use crate::fixed::facility::Identifier;

// TODO: the frontend dont care for internals like fittings.
// Split up typings and have one internal and one external?
// For now… just share everything witht the frontend.

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum SiteEntity {
    Facility(Facility),
    Lifeless(Lifeless),
    Npc(Npc),
    Player(Player),
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "SiteEntityFacility")]
pub struct Facility {
    pub id: Identifier,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "SiteEntityLifeless")]
pub struct Lifeless {
    pub id: String,
    // TODO: status like hitpoints?
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "SiteEntityNpc")]
pub struct Npc {
    pub shiplayout: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "SiteEntityPlayer")]
pub struct Player {
    pub id: String,
    pub shiplayout: String,
}

#[cfg(test)]
ts_rs::export! {
    Facility => "site-entity-facility.ts",
    Lifeless => "site-entity-lifeless.ts",
    Npc => "site-entity-npc.ts",
    Player => "site-entity-player.ts",
}

#[test]
fn can_parse() -> anyhow::Result<()> {
    let origin = SiteEntity::Lifeless(Lifeless {
        id: "lifelessAsteroid".to_string(),
    });
    let json = serde_json::to_string_pretty(&origin)?;
    println!("json {}", json);

    let some = serde_json::from_str::<SiteEntity>(&json)?;
    println!("some {:?}", some);

    if let SiteEntity::Lifeless(v) = some {
        assert_eq!(v.id, "lifelessAsteroid");
        Ok(())
    } else {
        panic!();
    }
}
