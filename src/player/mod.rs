use anyhow::anyhow;
use serde::{Deserialize, Serialize};

mod general;
pub mod location;
mod notifications;
mod station_assets;

pub use general::General;
pub use notifications::Notifications;
pub use station_assets::StationAssets;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS), ts(export))]
#[serde(rename_all = "camelCase", tag = "platform", content = "id")]
pub enum Player {
    Telegram(i64),
}

impl std::str::FromStr for Player {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splitted = s.split('-');
        let platform = splitted.next().ok_or_else(|| anyhow!("needs platform"))?;
        let id = splitted.next().ok_or_else(|| anyhow!("needs id"))?;
        if splitted.next().is_some() {
            return Err(anyhow!("can only contain exactly one dash (-)"));
        }
        match platform {
            "telegram" => Ok(Self::Telegram(id.parse::<i64>()?)),
            _ => Err(anyhow!("unknown player platform {} {}", platform, s)),
        }
    }
}

impl ToString for Player {
    fn to_string(&self) -> String {
        match self {
            Self::Telegram(id) => format!("telegram-{}", id),
        }
    }
}

#[test]
fn can_serde_parse_telegram_player() {
    let data = Player::Telegram(666);
    crate::test_helper::can_serde_parse(&data);
}

#[test]
fn can_string_parse_telegram_player() {
    let data = Player::Telegram(666);
    crate::test_helper::can_string_parse(&data);
}
