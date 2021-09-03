#![allow(clippy::module_name_repetitions)]

use serde::{Deserialize, Serialize};

use crate::fixed::item::Item;
use crate::fixed::solarsystem::Solarsystem;
use crate::market::{Order, Trader};
use crate::player::Player;

#[cfg(feature = "typescript")]
ts_rs::export! {
    PlaceOrder,
    TransferItems,
    Instruction => "typescript/generated-station-instruction.ts"
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[serde(
    rename_all = "camelCase",
    rename = "StationInstruction",
    tag = "type",
    content = "args"
)]
pub enum Instruction {
    // Ships
    /// Switches the `current_ship` with the selected one
    SwitchShip(usize),

    // Current ship
    Repair,
    Undock,
    LoadItemsIntoShip(TransferItems),
    UnloadItemsFromShip(TransferItems),

    // Unrelated from ships
    Buy(PlaceOrder),
    Sell(PlaceOrder),
    Recycle {
        item: Item,
        amount: u32,
    },
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct PlaceOrder {
    pub item: Item,
    pub amount: u32,
    pub paperclips: u64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct TransferItems {
    pub item: Item,
    pub amount: u32,
}

impl PlaceOrder {
    #[must_use]
    pub fn to_order(&self, player: Player, solarsystem: Solarsystem, station: u8) -> (Item, Order) {
        let order = Order::new_now(
            solarsystem,
            station,
            Trader::Player(player),
            self.amount,
            self.paperclips,
        );
        (self.item, order)
    }
}

#[test]
fn can_parse_undock() {
    let data = Instruction::Undock;
    crate::test_helper::can_serde_parse(&data);
}

#[test]
fn can_parse_repair() {
    let data = Instruction::Repair;
    crate::test_helper::can_serde_parse(&data);
}

#[test]
fn can_parse_load_ship_cargo() {
    let data = Instruction::LoadItemsIntoShip(TransferItems {
        item: Item::EXAMPLE,
        amount: 42,
    });
    crate::test_helper::can_serde_parse(&data);
}

#[test]
fn can_parse_buy() {
    let data = Instruction::Buy(PlaceOrder {
        item: Item::EXAMPLE,
        amount: 42,
        paperclips: 666,
    });
    crate::test_helper::can_serde_parse(&data);
}

#[test]
fn can_parse_recycle() {
    let data = Instruction::Recycle {
        item: Item::EXAMPLE,
        amount: 42,
    };
    crate::test_helper::can_serde_parse(&data);
}
