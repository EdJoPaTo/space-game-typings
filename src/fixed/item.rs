use serde::{Deserialize, Serialize};

use super::module;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", tag = "kind", content = "item")]
pub enum Item {
    ModulePassive(module::passive::Passive),
    ModuleTargeted(module::targeted::Targeted),
    ModuleUntargeted(module::untargeted::Untargeted),
    Ore(Ore),
}

/// Different types of ore.
/// These are proper names so they are not in camelCase or something like that.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub enum Ore {
    Aromit,
    Solmit,
    Tormit,
    Vesmit,
}

#[cfg(feature = "typescript")]
ts_rs::export! {
    Item => "item.ts",
    Ore => "item-ore.ts",
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
