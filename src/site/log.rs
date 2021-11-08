use serde::{Deserialize, Serialize};

use crate::fixed::facility::Facility;
use crate::fixed::item::Ore;
use crate::fixed::module::Targeted;
use crate::fixed::npc_faction::NpcFaction;
use crate::fixed::shiplayout::ShipLayout;
use crate::player::Player;

use super::Entity;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[serde(
    rename_all = "camelCase",
    rename = "SiteLog",
    tag = "type",
    content = "details"
)]
pub enum Log {
    ModuleTargeted((Actor, Targeted, Actor)),

    Collapse(Actor),
    Jump(Actor),
    RapidUnscheduledDisassembly(Actor),

    Dock(Actor),
    Undock(Actor),

    WarpIn(Actor),
    WarpOut(Actor),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "SiteLogActor", untagged)]
pub enum Actor {
    Asteroid(Ore),
    Facility(Facility),
    Npc((NpcFaction, ShipLayout)),
    Player((Player, ShipLayout)),
}

impl From<&Entity> for Actor {
    fn from(entity: &Entity) -> Self {
        match entity {
            Entity::Facility(info) => Self::Facility(*info),
            Entity::Asteroid(info) => Self::Asteroid(info.ore),
            Entity::Npc((f, s)) => Self::Npc((*f, s.fitting.layout)),
            Entity::Player((p, s)) => Self::Player((*p, s.fitting.layout)),
        }
    }
}

#[test]
fn can_parse_asteroid() {
    let data = Actor::Asteroid(Ore::Aromit);
    crate::test_helper::can_serde_parse(&data);
}

#[test]
fn can_parse_facility() {
    let data = Actor::Facility(Facility::Station);
    crate::test_helper::can_serde_parse(&data);
}

#[test]
fn can_parse_npc() {
    let data = Actor::Npc((NpcFaction::Pirates, ShipLayout::default()));
    crate::test_helper::can_serde_parse(&data);
}

#[test]
fn can_parse_player() {
    let data = Actor::Player((Player::Telegram(666), ShipLayout::default()));
    crate::test_helper::can_serde_parse(&data);
}
