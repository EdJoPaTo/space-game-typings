use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::fixed::npc_faction::NpcFaction;
use crate::fixed::solarsystem::Solarsystem;
use crate::player::Player;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS), ts(export))]
#[serde(rename_all = "camelCase", untagged)]
pub enum Trader {
    Npc(NpcFaction),
    Player(Player),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS), ts(export))]
#[serde(rename_all = "camelCase")]
pub struct Order {
    /// Time when the order was placed. Older orders with the same price are always handled first.
    #[cfg_attr(
        feature = "ts-rs",
        ts(type = "`${number}-${number}-${number}T${number}:${number}:${number}${string}Z`")
    )]
    pub date: DateTime<Utc>,

    /// Identifies the location of the order (together with the station index).
    pub solarsystem: Solarsystem,
    /// Identifies the location of the order (together with the solarsystem).
    pub station: u8,

    /// The one that has issued the order
    pub trader: Trader,

    /// Amount of items. Order can be partially fulfilled.
    /// Total Price is calculated by `amount` * `paperclips`.
    pub amount: u32,

    /// Paperclips are the currency.
    /// Total Price is calculated by `amount` * `paperclips`.
    pub paperclips: u64,
}

impl Order {
    #[must_use]
    pub fn new_now(
        solarsystem: Solarsystem,
        station: u8,
        trader: Trader,
        amount: u32,
        paperclips: u64,
    ) -> Self {
        Self {
            date: Utc::now(),
            solarsystem,
            station,
            trader,
            amount,
            paperclips,
        }
    }

    #[cfg(test)]
    #[must_use]
    pub(super) fn new_test(
        date: &str,
        solarsystem: Solarsystem,
        amount: u32,
        paperclips: u64,
    ) -> Self {
        Self {
            date: DateTime::parse_from_rfc3339(date).unwrap().into(),
            solarsystem,
            station: 0,
            trader: Trader::Npc(NpcFaction::Guards),
            amount,
            paperclips,
        }
    }

    #[cfg(test)]
    #[must_use]
    pub(super) fn new_test_simple(amount: u32, paperclips: u64) -> Self {
        Self::new_test(
            "1997-12-19T16:53:14Z",
            Solarsystem::default(),
            amount,
            paperclips,
        )
    }

    #[must_use]
    pub const fn reduce_to(&self, remaining: u32) -> Self {
        Self {
            amount: remaining,
            ..*self
        }
    }

    #[must_use]
    pub const fn reduce_by(&self, amount: u32) -> Self {
        Self {
            amount: self.amount - amount,
            ..*self
        }
    }

    #[must_use]
    pub const fn total_paperclips(&self) -> u64 {
        let amount = self.amount as u64;
        self.paperclips.saturating_mul(amount)
    }

    #[must_use]
    pub const fn is_valid(&self) -> bool {
        if self.amount == 0 || self.paperclips == 0 {
            return false;
        }
        let amount = self.amount as u64;
        if amount.checked_mul(self.paperclips).is_none() {
            return false;
        }
        true
    }

    #[cfg(test)]
    #[must_use]
    pub(super) fn example_a() -> Self {
        Self::new_test("2000-12-19T15:00:00Z", Solarsystem::default(), 42, 666)
    }
    #[cfg(test)]
    #[must_use]
    pub(super) fn example_b() -> Self {
        Self::new_test("1996-12-19T15:00:00Z", Solarsystem::default(), 1337, 666)
    }
    #[cfg(test)]
    #[must_use]
    pub(super) fn example_c() -> Self {
        Self::new_test("2010-12-19T15:00:00Z", Solarsystem::default(), 666, 42)
    }
}

impl From<Player> for Trader {
    fn from(player: Player) -> Self {
        Self::Player(player)
    }
}

impl From<NpcFaction> for Trader {
    fn from(faction: NpcFaction) -> Self {
        Self::Npc(faction)
    }
}

#[test]
fn can_serde_parse_trader_npc() {
    let data = Trader::Npc(NpcFaction::Pirates);
    crate::test_helper::can_serde_parse(&data);
}

#[test]
fn can_serde_parse_trader_player() {
    let data = Trader::Player(Player::Telegram(666));
    crate::test_helper::can_serde_parse(&data);
}

#[test]
fn can_serde_parse_order() {
    let data = Order {
        date: DateTime::parse_from_rfc3339("1996-12-19T16:57:12.421337Z")
            .unwrap()
            .into(),
        solarsystem: Solarsystem::default(),
        station: 42,
        trader: Trader::Npc(NpcFaction::Guards),
        paperclips: 666,
        amount: 1337,
    };
    crate::test_helper::can_serde_parse(&data);
}
