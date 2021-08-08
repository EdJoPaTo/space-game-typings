use serde::{Deserialize, Serialize};

use crate::fixed::npc_faction::NpcFaction;
use crate::fixed::{facility, lifeless, shiplayout};

use super::player;
use super::ship::{Fitting, Status};

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
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Npc {
    pub faction: NpcFaction,
    pub fitting: Fitting,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub id: player::Identifier,
    pub shiplayout: shiplayout::Identifier,
}

#[test]
fn can_parse() {
    let data = SiteEntity::Lifeless(Lifeless {
        id: "lifelessAsteroid".to_string(),
        status: Status {
            capacitor: 0,
            hitpoints_armor: 42,
            hitpoints_structure: 42,
        },
    });
    crate::test_helper::can_serde_parse(&data);
}
