use std::str::FromStr;

use serde::{Deserialize, Serialize};

use super::module;

mod details;
mod mineral;
mod ore;

pub use details::{Category, Details};
pub use mineral::Mineral;
pub use ore::Ore;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", untagged)]
pub enum Item {
    Mineral(Mineral),
    ModulePassive(module::passive::Passive),
    ModuleTargeted(module::targeted::Targeted),
    ModuleUntargeted(module::untargeted::Untargeted),
    Ore(Ore),
}

#[cfg(feature = "typescript")]
ts_rs::export! {
    Category,
    Details,
    Mineral,
    Ore,
    Item => "typescript/generated-item.ts"
}

impl From<Mineral> for Item {
    fn from(mineral: Mineral) -> Self {
        Self::Mineral(mineral)
    }
}

impl From<module::passive::Passive> for Item {
    fn from(i: module::passive::Passive) -> Self {
        Self::ModulePassive(i)
    }
}

impl From<module::targeted::Targeted> for Item {
    fn from(i: module::targeted::Targeted) -> Self {
        Self::ModuleTargeted(i)
    }
}

impl From<module::untargeted::Untargeted> for Item {
    fn from(i: module::untargeted::Untargeted) -> Self {
        Self::ModuleUntargeted(i)
    }
}

impl From<Ore> for Item {
    fn from(ore: Ore) -> Self {
        Self::Ore(ore)
    }
}

impl FromStr for Item {
    type Err = serde_json::Error;
    /// Naively implemented via `serde_json`. Its a bit ugly but works for now.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(&format!(r#""{}""#, s))
    }
}
impl ToString for Item {
    /// Naively implemented via `serde_json`. Its a bit ugly but works for now.
    fn to_string(&self) -> String {
        let str = serde_json::to_string(self).unwrap();
        str.trim_matches('"').to_string()
    }
}

impl Item {
    #[cfg(test)]
    pub(crate) const EXAMPLE: Self = Self::Ore(Ore::Solmit);
}

#[test]
fn can_serde_parse_mineral() {
    let data = Item::Mineral(Mineral::Derite);
    crate::test_helper::can_serde_parse(&data);
}

#[test]
fn can_string_parse_mineral() {
    let data = Item::Mineral(Mineral::Derite);
    crate::test_helper::can_string_parse(&data);
}

#[test]
fn can_serde_parse_module() {
    let data = Item::ModulePassive(module::passive::Passive::RookieArmorPlate);
    crate::test_helper::can_serde_parse(&data);
}

#[test]
fn can_string_parse_module() {
    let data = Item::ModulePassive(module::passive::Passive::RookieArmorPlate);
    crate::test_helper::can_string_parse(&data);
}

#[test]
fn can_serde_parse_ore() {
    let data = Item::Ore(Ore::Aromit);
    crate::test_helper::can_serde_parse(&data);
}

#[test]
fn can_string_parse_ore() {
    let data = Item::Ore(Ore::Aromit);
    crate::test_helper::can_string_parse(&data);
}

#[test]
fn can_parse_hash_map_with_item_as_key() {
    let mut data = std::collections::HashMap::new();
    data.insert(Item::Ore(Ore::Aromit), 42);
    crate::test_helper::can_serde_parse(&data);
}
