use serde::{Deserialize, Serialize};

use super::module;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", tag = "kind", content = "item")]
pub enum Item {
    ModulePassive(module::passive::Passive),
    ModuleTargeted(module::targeted::Targeted),
    ModuleUntargeted(module::untargeted::Untargeted),
    Ore,
}

#[cfg(feature = "typescript")]
ts_rs::export! {
    Item => "item.ts",
}

#[test]
fn can_parse_module() {
    let data = Item::ModulePassive(module::passive::Passive::RookieArmorPlate);
    crate::test_helper::can_serde_parse(&data);
}

#[test]
fn can_parse_ore() {
    let data = Item::Ore;
    crate::test_helper::can_serde_parse(&data);
}
