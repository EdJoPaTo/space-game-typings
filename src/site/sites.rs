use std::collections::BTreeMap;

use anyhow::anyhow;
use serde::{Deserialize, Serialize};

use crate::fixed::solarsystem::Solarsystem;

// TODO: move to crate::site

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS), ts(export))]
#[serde(rename_all = "camelCase", tag = "kind", content = "unique")]
pub enum Site {
    /// Zero-based index of station. Station I is 0, station IV is 3.
    Station(u8),
    /// Target solarsystem
    Stargate(Solarsystem),
    /// unique number, maybe random
    AsteroidField(u8),
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS), ts(export))]
pub struct SitesNearPlanet(BTreeMap<u8, Vec<Site>>);

impl std::str::FromStr for Site {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splitted = s.split('-');
        let kind = splitted.next().ok_or_else(|| anyhow!("needs kind"))?;
        let unique = splitted.next().ok_or_else(|| anyhow!("needs unique"))?;
        if splitted.next().is_some() {
            return Err(anyhow!("can only contain exactly one dash (-)"));
        }
        match kind {
            "station" => Ok(Self::Station(unique.parse()?)),
            "stargate" => Ok(Self::Stargate(unique.parse()?)),
            "asteroidField" => Ok(Self::AsteroidField(unique.parse()?)),
            _ => Err(anyhow!("unknown site kind {} {}", kind, s)),
        }
    }
}

impl ToString for Site {
    fn to_string(&self) -> String {
        match self {
            Self::Station(index) => format!("station-{}", index),
            Self::Stargate(target) => format!("stargate-{}", target),
            Self::AsteroidField(unique) => format!("asteroidField-{:03}", unique),
        }
    }
}

impl SitesNearPlanet {
    #[must_use]
    pub fn all(&self) -> Vec<Site> {
        self.0.values().flatten().copied().collect()
    }

    pub fn add(&mut self, planet: u8, site: Site) {
        let vec = self.0.entry(planet).or_default();
        vec.push(site);
        vec.sort();
    }

    pub fn remove(&mut self, site: Site) {
        for sites in self.0.values_mut() {
            if let Some(position) = sites.iter().position(|o| o == &site) {
                sites.remove(position);
            }
        }
    }
}

#[test]
fn can_serde_parse_station() {
    let data = Site::Station(2);
    crate::test_helper::can_serde_parse(&data);
}

#[test]
fn can_serde_parse_stargate() {
    let data = Site::Stargate(Solarsystem::Wabinihwa);
    crate::test_helper::can_serde_parse(&data);
}

#[test]
fn can_serde_parse_asteroid_field() {
    let data = Site::AsteroidField(42);
    crate::test_helper::can_serde_parse(&data);
}

#[test]
fn can_string_parse_station() {
    let data = Site::Station(2);
    crate::test_helper::can_string_parse(&data);
}

#[test]
fn can_string_parse_stargate() {
    let data = Site::Stargate(Solarsystem::Wabinihwa);
    crate::test_helper::can_string_parse(&data);
}

#[test]
fn can_string_parse_asteroid_field() {
    let data = Site::AsteroidField(42);
    crate::test_helper::can_string_parse(&data);
}
