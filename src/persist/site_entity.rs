use serde::{Deserialize, Serialize};

use crate::entity::Collateral;
use crate::fixed::facility::Facility;
use crate::fixed::npc_faction::NpcFaction;
use crate::fixed::{lifeless, LifelessThingies};
use crate::serde_helper::is_default;
use crate::ship::Ship;

use super::player::Player;

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
    pub collateral: Collateral,

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
            collateral: Collateral {
                capacitor: 0,
                armor: details.collateral.armor,
                structure: details.collateral.structure,
            },
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
    let data = SiteEntity::Facility(Facility::Stargate);
    crate::test_helper::can_serde_parse(&data);
}

#[test]
fn can_parse_lifeless() {
    let data = SiteEntity::Lifeless(Lifeless {
        id: lifeless::Lifeless::Asteroid,
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
