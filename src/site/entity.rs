use serde::{Deserialize, Serialize};

use crate::entity::Collateral;
use crate::fixed::facility::Facility;
use crate::fixed::item::Ore;
use crate::fixed::npc_faction::NpcFaction;
use crate::player::Player;
use crate::ship::Ship;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum Entity {
    Asteroid(EntityAsteroid),
    Facility(Facility),
    Npc((NpcFaction, Ship)),
    Player((Player, Ship)),
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct EntityAsteroid {
    pub collateral: Collateral,
    pub max_structure: u16,
    pub ore: Ore,
    pub remaining_ore: u32,
}

impl Entity {
    #[must_use]
    pub fn new_asteroid(ore: Ore, remaining_ore: u32, structure: u16) -> Self {
        let collateral = Collateral::new_structure(structure);
        Self::Asteroid(EntityAsteroid {
            collateral,
            max_structure: structure,
            ore,
            remaining_ore,
        })
    }
}

impl EntityAsteroid {
    #[must_use]
    pub fn is_collapsed(&self) -> bool {
        !self.collateral.is_alive() || self.remaining_ore == 0
    }
}

#[test]
fn can_parse_asteroid() {
    let data = Entity::new_asteroid(Ore::Aromit, 42, 42);
    crate::test_helper::can_serde_parse(&data);
}

#[test]
fn can_parse_facility() {
    let data = Entity::Facility(Facility::Stargate);
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
