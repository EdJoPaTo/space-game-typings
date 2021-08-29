use std::fmt::Display;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

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

impl Display for Ore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for Ore {
    type Err = serde_json::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(&format!(r#""{}""#, s))
    }
}

#[test]
fn can_serde_parse() {
    let data = Ore::Aromit;
    crate::test_helper::can_serde_parse(&data);
}

#[test]
fn can_string_parse() {
    let data = Ore::Aromit;
    crate::test_helper::can_string_parse(&data);
}
