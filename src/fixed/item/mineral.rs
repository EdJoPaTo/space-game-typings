use std::fmt::Display;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

/// Minerals are the basic components needed to build stuff.
/// These are proper names so they are not in camelCase or something like that.
/// Naming scheme: ends with 'ite' (derived from the Greek word 'lithos' just like human minerals).
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub enum Mineral {
    Derite,
    Fylite,
    Ragite,
}

impl Display for Mineral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for Mineral {
    type Err = serde_json::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(&format!(r#""{}""#, s))
    }
}

#[test]
fn can_serde_parse() {
    let data = Mineral::Derite;
    crate::test_helper::can_serde_parse(&data);
}

#[test]
fn can_string_parse() {
    let data = Mineral::Derite;
    crate::test_helper::can_string_parse(&data);
}
