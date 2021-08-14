use serde::{Deserialize, Serialize};

use crate::fixed::facility::Facility;
use crate::fixed::lifeless;
use crate::fixed::npc_faction::NpcFaction;

use super::player::Player;
use super::ship::{Fitting, Status};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum SiteEntity {
    Facility(Facility),
    Lifeless(Lifeless),
    Npc(Npc),
    Player(Player),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Lifeless {
    pub id: lifeless::Lifeless,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Npc {
    pub faction: NpcFaction,
    pub fitting: Fitting,
    pub status: Status,
}

#[test]
fn can_parse() {
    let data = SiteEntity::Lifeless(Lifeless {
        id: lifeless::Lifeless::Asteroid,
        status: Status {
            capacitor: 0,
            hitpoints_armor: 42,
            hitpoints_structure: 42,
        },
    });
    crate::test_helper::can_serde_parse(&data);
}
