use serde::{Deserialize, Serialize};

use crate::entity::Collateral;
use crate::fixed::facility::Facility;
use crate::fixed::lifeless::Lifeless;
use crate::fixed::npc_faction::NpcFaction;
use crate::fixed::LifelessThingies;
use crate::player::Player;
use crate::serde_helper::is_default;
use crate::ship::Ship;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum Entity {
    Facility(Facility),
    Lifeless(EntityLifeless),
    Npc((NpcFaction, Ship)),
    Player((Player, Ship)),
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct EntityLifeless {
    pub id: Lifeless,
    pub collateral: Collateral,

    #[serde(default, skip_serializing_if = "is_default")]
    pub remaining_ore: u16,
}

impl EntityLifeless {
    #[must_use]
    pub fn new(statics: &LifelessThingies, lifeless: Lifeless) -> Self {
        let details = statics.get(&lifeless);
        Self {
            id: lifeless,
            collateral: details.collateral,
            remaining_ore: details.ore,
        }
    }

    /// States if the entity has no point anymore and can be removed.
    /// Asteroid has no ore anymore, Wreck has no loot, ...
    #[must_use]
    pub const fn is_collapsed(&self) -> bool {
        if !self.collateral.is_alive() {
            return true;
        }

        self.remaining_ore == 0
    }
}

#[test]
fn can_parse_facility() {
    let data = Entity::Facility(Facility::Stargate);
    crate::test_helper::can_serde_parse(&data);
}

#[test]
fn can_parse_lifeless() {
    let data = Entity::Lifeless(EntityLifeless {
        id: Lifeless::Asteroid,
        collateral: Collateral {
            capacitor: 0,
            armor: 42,
            structure: 42,
        },
        remaining_ore: 42,
    });
    crate::test_helper::can_serde_parse(&data);
}

#[test]
fn can_parse_npc() {
    let data = Entity::Npc((NpcFaction::Pirates, Ship::default()));
    crate::test_helper::can_serde_parse(&data);
}

#[test]
fn can_parse_player() {
    let data = Entity::Player((Player::Telegram(666), Ship::default()));
    crate::test_helper::can_serde_parse(&data);
}
