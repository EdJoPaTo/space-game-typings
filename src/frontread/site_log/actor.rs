use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::fixed::facility::Facility;
use crate::fixed::lifeless::Lifeless;
use crate::fixed::npc_faction::NpcFaction;
use crate::fixed::shiplayout::ShipLayout;
use crate::persist::player::Player;
use crate::persist::ship::Ship;
use crate::persist::site_entity::SiteEntity;

type Npc = (NpcFaction, ShipLayout);
type PlayerLayout = (Player, ShipLayout);

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", untagged)]
pub enum SiteLogActor {
    Facility(Facility),
    Lifeless(Lifeless),
    Player(PlayerLayout),
    Npc(Npc),
}

impl SiteLogActor {
    #[must_use]
    pub fn from(player_ships: &HashMap<Player, Ship>, entity: &SiteEntity) -> Self {
        match entity {
            SiteEntity::Facility(info) => SiteLogActor::Facility(*info),
            SiteEntity::Lifeless(info) => SiteLogActor::Lifeless(info.id),
            SiteEntity::Npc(info) => SiteLogActor::Npc((info.faction, info.fitting.layout)),
            SiteEntity::Player(player) => {
                let layout = player_ships
                    .get(player)
                    .map(|o| o.fitting.layout)
                    .unwrap_or_default();
                SiteLogActor::Player((*player, layout))
            }
        }
    }
}

#[cfg(test)]
ts_rs::export! {
    SiteLogActor => "site-log-actor.ts",
}

#[test]
fn can_parse_facility() {
    let data = SiteLogActor::Facility(Facility::Station);
    crate::test_helper::can_serde_parse(&data);
}

#[test]
fn can_parse_lifeless() {
    let data = SiteLogActor::Lifeless(Lifeless::Asteroid);
    crate::test_helper::can_serde_parse(&data);
}

#[test]
fn can_parse_player() {
    let data = SiteLogActor::Player((Player::Telegram(666), ShipLayout::default()));
    crate::test_helper::can_serde_parse(&data);
}

#[test]
fn can_parse_npc() {
    let data = SiteLogActor::Npc((NpcFaction::Pirates, ShipLayout::default()));
    crate::test_helper::can_serde_parse(&data);
}
