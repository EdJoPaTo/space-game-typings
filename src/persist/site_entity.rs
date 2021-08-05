use serde::{Deserialize, Serialize};

use crate::fixed::npc_faction::NpcFaction;
use crate::fixed::{facility, lifeless, shiplayout};

use super::player;
use super::ship::Ship;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum SiteEntity {
    Facility(Facility),
    Lifeless(Lifeless),
    Npc(Npc),
    Player(Player),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub struct Facility {
    pub id: facility::Identifier,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Lifeless {
    pub id: lifeless::Identifier,
    // TODO: status like hitpoints?
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Npc {
    pub faction: NpcFaction,
    pub ship: Ship,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub id: player::Identifer,
    pub shiplayout: shiplayout::Identifier,
}

#[test]
fn can_parse() {
    let data = SiteEntity::Lifeless(Lifeless {
        id: "lifelessAsteroid".to_string(),
    });
    crate::test_helper::can_serde_parse(&data);
}
