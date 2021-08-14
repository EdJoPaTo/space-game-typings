use anyhow::anyhow;
use serde::{Deserialize, Serialize};

use crate::fixed::solarsystem::Solarsystem;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", tag = "platform", content = "id")]
pub enum Player {
    Telegram(i64),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename = "PlayerGeneral")]
pub struct General {
    pub home_solarsystem: Solarsystem,
    pub home_station: u8,

    /// Paperclips are the currency
    pub paperclips: u64,
}

impl Default for General {
    fn default() -> Self {
        Self {
            home_solarsystem: Solarsystem::default(),
            home_station: 0,
            paperclips: 2000,
        }
    }
}

#[cfg(test)]
ts_rs::export! {
    Player => "player.ts",
    General => "player-general.ts",
}

impl std::str::FromStr for Player {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splitted = s.split('-');
        let platform = splitted
            .next()
            .ok_or_else(|| anyhow!("has to contain platform"))?;
        let id = splitted
            .next()
            .ok_or_else(|| anyhow!("has to contain id"))?;
        if splitted.next().is_some() {
            return Err(anyhow!("can only contain exactly one dash (-)"));
        }
        match platform {
            "telegram" => Ok(Player::Telegram(id.parse::<i64>()?)),
            _ => Err(anyhow!("unknown player platform {} {}", platform, s)),
        }
    }
}

impl ToString for Player {
    fn to_string(&self) -> String {
        match self {
            Player::Telegram(id) => format!("telegram-{}", id),
        }
    }
}

#[test]
fn can_identify_telegram_player() {
    let data = Player::Telegram(666);
    crate::test_helper::can_serde_parse(&data);
}
