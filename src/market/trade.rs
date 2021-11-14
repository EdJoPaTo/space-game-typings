use serde::{Deserialize, Serialize};

use crate::fixed::solarsystem::Solarsystem;

use super::{Order, Trader};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS), ts(export))]
#[serde(rename = "Trade")]
pub struct Trade {
    pub solarsystem: Solarsystem,
    pub station: u8,

    pub buyer: Trader,
    pub seller: Trader,

    pub amount: u32,
    /// Paperclips per item.
    /// Total Price is calculated by `amount` * `paperclips`.
    pub paperclips: u64,
}

impl Trade {
    #[cfg(test)]
    #[must_use]
    pub fn new_test(amount: u32, paperclips: u64) -> Self {
        use crate::fixed::npc_faction::NpcFaction;
        Self {
            solarsystem: Solarsystem::default(),
            station: 0,
            buyer: Trader::Npc(NpcFaction::Guards),
            seller: Trader::Npc(NpcFaction::Guards),
            amount,
            paperclips,
        }
    }

    #[must_use]
    pub fn resolve(buy: Order, sell: Order) -> Option<(Self, Order, Order)> {
        if buy.solarsystem != sell.solarsystem
            || buy.station != sell.station
            || buy.paperclips < sell.paperclips
            || buy.amount == 0
            || sell.amount == 0
        {
            return None;
        }
        let amount = buy.amount.min(sell.amount);
        let trade = Self {
            solarsystem: sell.solarsystem,
            station: sell.station,
            buyer: buy.trader,
            seller: sell.trader,
            amount,
            paperclips: buy.paperclips.min(sell.paperclips),
        };
        let remaining_buy = buy.reduce_by(amount);
        let remaining_sell = sell.reduce_by(amount);
        Some((trade, remaining_buy, remaining_sell))
    }

    #[must_use]
    pub const fn total_paperclips(&self) -> u64 {
        let amount = self.amount as u64;
        self.paperclips.saturating_mul(amount)
    }
}

#[test]
fn can_serde_parse_trade() {
    let data = Trade::new_test(42, 666);
    crate::test_helper::can_serde_parse(&data);
}

#[test]
fn solarsystem_doesnt_trade() {
    let buy = Order::new_test("1997-12-27T16:00:00Z", Solarsystem::Iramil, 5, 666);
    let sell = Order::new_test("1997-12-27T16:00:00Z", Solarsystem::Arama, 666, 1);
    let trade = Trade::resolve(buy, sell);
    assert_eq!(trade, None);
}

#[test]
fn station_doesnt_trade() {
    let mut buy = Order::new_test_simple(5, 666);
    let sell = Order::new_test_simple(666, 1);
    buy.station = 42;

    let trade = Trade::resolve(buy, sell);
    assert_eq!(trade, None);
}

#[test]
fn price_doesnt_trade() {
    let buy = Order::new_test_simple(5, 20);
    let sell = Order::new_test_simple(5, 40);
    let trade = Trade::resolve(buy, sell);
    assert_eq!(trade, None);
}

#[test]
fn empty_buy_doesnt_trade() {
    let buy = Order::new_test_simple(0, 20);
    let sell = Order::new_test_simple(5, 20);
    let trade = Trade::resolve(buy, sell);
    assert_eq!(trade, None);
}

#[test]
fn empty_sell_doesnt_trade() {
    let buy = Order::new_test_simple(5, 20);
    let sell = Order::new_test_simple(0, 20);
    let trade = Trade::resolve(buy, sell);
    assert_eq!(trade, None);
}

#[test]
fn exact_trade() {
    let buy = Order::new_test_simple(5, 20);
    let sell = Order::new_test_simple(5, 20);
    let (trade, remaining_buy, remaining_sell) = Trade::resolve(buy, sell).unwrap();
    assert_eq!(trade, Trade::new_test(5, 20));
    assert_eq!(remaining_buy.amount, 0);
    assert_eq!(remaining_sell.amount, 0);
}

#[test]
fn trade_with_remaining_buy() {
    let buy = Order::new_test_simple(10, 20);
    let sell = Order::new_test_simple(2, 20);
    let (trade, remaining_buy, remaining_sell) = Trade::resolve(buy, sell).unwrap();
    assert_eq!(trade, Trade::new_test(2, 20));
    assert_eq!(remaining_buy, buy.reduce_to(8));
    assert_eq!(remaining_sell.amount, 0);
}

#[test]
fn trade_with_remaining_sell() {
    let buy = Order::new_test_simple(2, 20);
    let sell = Order::new_test_simple(10, 20);
    let (trade, remaining_buy, remaining_sell) = Trade::resolve(buy, sell).unwrap();
    assert_eq!(trade, Trade::new_test(2, 20));
    assert_eq!(remaining_buy.amount, 0);
    assert_eq!(remaining_sell, sell.reduce_to(8));
}

#[test]
fn trade_uses_cheaper_price() {
    let buy = Order::new_test_simple(5, 30);
    let sell = Order::new_test_simple(5, 10);
    let (trade, remaining_buy, remaining_sell) = Trade::resolve(buy, sell).unwrap();
    assert_eq!(trade, Trade::new_test(5, 10));
    assert_eq!(remaining_buy.amount, 0);
    assert_eq!(remaining_sell.amount, 0);
}

#[test]
fn has_correct_buyer_seller() {
    use crate::fixed::npc_faction::NpcFaction;
    let buy = Order::new_now(
        Solarsystem::default(),
        0,
        Trader::Npc(NpcFaction::Pirates),
        5,
        20,
    );
    let sell = Order::new_test_simple(5, 20);
    let (trade, _remaining_buy, _remaining_sell) = Trade::resolve(buy, sell).unwrap();
    assert_eq!(trade.buyer, Trader::Npc(NpcFaction::Pirates));
    assert_eq!(trade.seller, Trader::Npc(NpcFaction::Guards));
}
