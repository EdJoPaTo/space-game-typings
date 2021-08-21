use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub enum StationInstruction {
    Repair,
    Undock,
    SellOre,
}

#[cfg(feature = "typescript")]
ts_rs::export! {
    StationInstruction => "station-instruction.ts",
}

#[test]
fn can_identify_undock() {
    let data = StationInstruction::Undock;
    crate::test_helper::can_serde_parse(&data);
}

#[test]
fn can_identify_repair() {
    let data = StationInstruction::Repair;
    crate::test_helper::can_serde_parse(&data);
}
