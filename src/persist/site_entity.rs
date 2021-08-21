use serde::{Deserialize, Serialize};

use crate::fixed::facility::Facility;
use crate::fixed::npc_faction::NpcFaction;
use crate::fixed::{lifeless, LifelessThingies};
use crate::serde_helper::is_default;

use super::player::Player;
use super::ship::{Ship, Status};

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

    #[serde(default, skip_serializing_if = "is_default")]
    pub remaining_ore: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Npc {
    pub faction: NpcFaction,

    #[serde(flatten)]
    pub ship: Ship,
}

impl Lifeless {
    #[must_use]
    pub fn new(statics: &LifelessThingies, lifeless: lifeless::Lifeless) -> Self {
        let details = statics.get(&lifeless);
        Self {
            id: lifeless,
            status: Status {
                capacitor: 0,
                hitpoints_armor: details.hitpoints_armor,
                hitpoints_structure: details.hitpoints_structure,
            },
            remaining_ore: details.ore,
        }
    }

    #[must_use]
    /// States if the entity has no point anymore and can be removed.
    /// Asteroid has no ore anymore, Wreck has no loot, ...
    pub const fn is_collapsed(&self) -> bool {
        if !self.status.is_alive() {
            return true;
        }

        self.remaining_ore == 0
    }
}

#[test]
fn can_parse_facility() {
    let data = SiteEntity::Facility(Facility::Stargate);
    crate::test_helper::can_serde_parse(&data);
}

#[test]
fn can_parse_lifeless() {
    let data = SiteEntity::Lifeless(Lifeless {
        id: lifeless::Lifeless::Asteroid,
        status: Status {
            capacitor: 0,
            hitpoints_armor: 42,
            hitpoints_structure: 42,
        },
        remaining_ore: 42,
    });
    crate::test_helper::can_serde_parse(&data);
}

#[test]
fn can_parse_player() {
    let data = SiteEntity::Player(Player::Telegram(666));
    crate::test_helper::can_serde_parse(&data);
}

#[test]
fn can_parse_npc() {
    let data = SiteEntity::Npc(Npc {
        faction: NpcFaction::Pirates,
        ship: Ship::default(),
    });
    crate::test_helper::can_serde_parse(&data);
}
