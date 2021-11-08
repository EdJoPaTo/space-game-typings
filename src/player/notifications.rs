use serde::{Deserialize, Serialize};

use crate::fixed::item::Item;
use crate::market::Trade;
use crate::site;

/// Stuff which happened in a game round.
#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "PlayerNotifications")]
pub struct Notifications {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub site_log: Vec<site::Log>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub trades: Vec<(Item, Trade)>,
}

impl From<Vec<site::Log>> for Notifications {
    fn from(log: Vec<site::Log>) -> Self {
        Self {
            site_log: log,
            ..Self::default()
        }
    }
}

impl From<(Item, Trade)> for Notifications {
    fn from(tuple: (Item, Trade)) -> Self {
        Self {
            trades: vec![tuple],
            ..Self::default()
        }
    }
}

impl Notifications {
    pub fn append(&mut self, other: &mut Self) {
        self.site_log.append(&mut other.site_log);
        self.trades.append(&mut other.trades);
    }
}

#[test]
fn can_serde_parse_empty() {
    let data = Notifications::default();
    crate::test_helper::can_serde_parse(&data);
}
