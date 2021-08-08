use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub enum StationInstruction {
    Repair,
    Undock,
}

#[cfg(test)]
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
