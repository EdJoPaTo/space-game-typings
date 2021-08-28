use serde::{Deserialize, Serialize};

use crate::entity::Collateral;
use crate::fixed::facility::Facility;
use crate::fixed::item::Ore;
use crate::fixed::lifeless::Lifeless;
use crate::fixed::npc_faction::NpcFaction;
use crate::fixed::LifelessThingies;
use crate::player::Player;
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

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minable: Option<(Ore, u32)>,
}

impl EntityLifeless {
    #[must_use]
    pub fn new(statics: &LifelessThingies, lifeless: Lifeless) -> Self {
        let details = statics.get(&lifeless);
        Self {
            id: lifeless,
            collateral: details.collateral,
            minable: details.minable,
        }
    }

    /// States if the entity has no point anymore and can be removed.
    /// Asteroid has no ore anymore, Wreck has no loot, ...
    #[allow(clippy::let_and_return)]
    #[must_use]
    pub fn is_collapsed(&self) -> bool {
        if !self.collateral.is_alive() {
            return true;
        }

        let is_minable = self.minable.map_or(false, |(_, amount)| amount > 0);

        is_minable
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
        minable: Some((Ore::Aromit, 42)),
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
