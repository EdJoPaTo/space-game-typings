#![allow(clippy::module_name_repetitions)]

use serde::{Deserialize, Serialize};

use crate::fixed::module::targeted::Targeted;

pub use self::actor::SiteLogActor;

mod actor;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", tag = "type", content = "details")]
pub enum SiteLog {
    ModuleTargeted((SiteLogActor, Targeted, SiteLogActor)),

    Jump(SiteLogActor),
    RapidUnscheduledDisassembly(SiteLogActor),

    Dock(SiteLogActor),
    Undock(SiteLogActor),

    WarpIn(SiteLogActor),
    WarpOut(SiteLogActor),
}

#[cfg(test)]
ts_rs::export! {
    SiteLog => "site-log.ts",
}
