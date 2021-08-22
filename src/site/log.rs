use serde::{Deserialize, Serialize};

use crate::fixed::facility::Facility;
use crate::fixed::lifeless::Lifeless;
use crate::fixed::module::targeted::Targeted;
use crate::fixed::npc_faction::NpcFaction;
use crate::fixed::shiplayout::ShipLayout;
use crate::persist::player::Player;

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
    Facility(Facility),
    Lifeless(Lifeless),
    Npc((NpcFaction, ShipLayout)),
    Player((Player, ShipLayout)),
}

#[cfg(feature = "typescript")]
ts_rs::export! {
    Actor => "site-log-actor.ts",
    Log => "site-log.ts",
}

impl From<&Entity> for Actor {
    fn from(entity: &Entity) -> Self {
        match entity {
            Entity::Facility(info) => Actor::Facility(*info),
            Entity::Lifeless(info) => Actor::Lifeless(info.id),
            Entity::Npc((f, s)) => Actor::Npc((*f, s.fitting.layout)),
            Entity::Player((p, s)) => Actor::Player((*p, s.fitting.layout)),
        }
    }
}

#[test]
fn can_parse_facility() {
    let data = Actor::Facility(Facility::Station);
    crate::test_helper::can_serde_parse(&data);
}

#[test]
fn can_parse_lifeless() {
    let data = Actor::Lifeless(Lifeless::Asteroid);
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
