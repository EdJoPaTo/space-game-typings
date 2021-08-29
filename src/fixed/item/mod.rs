use serde::{Deserialize, Serialize};

use super::module;

mod ore;

pub use ore::Ore;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", untagged)]
pub enum Item {
    ModulePassive(module::passive::Passive),
    ModuleTargeted(module::targeted::Targeted),
    ModuleUntargeted(module::untargeted::Untargeted),
    Ore(Ore),
}

#[cfg(feature = "typescript")]
ts_rs::export! {
    Ore,
    Item => "typescript/generated-item.ts"
}

impl From<Ore> for Item {
    fn from(ore: Ore) -> Self {
        Self::Ore(ore)
    }
}

#[test]
fn can_parse_module() {
    let data = Item::ModulePassive(module::passive::Passive::RookieArmorPlate);
    crate::test_helper::can_serde_parse(&data);
}

#[test]
fn can_parse_ore() {
    let data = Item::Ore(Ore::Aromit);
    crate::test_helper::can_serde_parse(&data);
}

#[test]
fn can_parse_hash_map_with_item_as_key() {
    let mut data = std::collections::HashMap::new();
    data.insert(Item::Ore(Ore::Aromit), 42);
    crate::test_helper::can_serde_parse(&data);
}