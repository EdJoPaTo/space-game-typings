use serde::{Deserialize, Serialize};

use crate::fixed::facility::Identifier;
use crate::fixed::npc_faction::NpcFaction;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum SiteEntity {
    Facility(Facility),
    Lifeless(Lifeless),
    Npc(Npc),
    Player(Player),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Facility {
    pub id: Identifier,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Lifeless {
    pub id: String,
    // TODO: status like hitpoints?
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Npc {
    pub faction: NpcFaction,
    pub shiplayout: String,
    // TODO: status like hitpoints?
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub id: String,
    pub shiplayout: String,
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
