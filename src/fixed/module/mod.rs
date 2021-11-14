use serde::{Deserialize, Serialize};

mod passive;
mod targeted;
mod untargeted;

pub use passive::Details as PassiveDetails;
pub use passive::Passive;
pub use targeted::Details as TargetedDetails;
pub use targeted::Targeted;
pub use untargeted::Details as UntargetedDetails;
pub use untargeted::Untargeted;

#[derive(Debug, Hash, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS), ts(export))]
#[serde(rename_all = "camelCase", untagged)]
pub enum Module {
    Passive(Passive),
    Targeted(Targeted),
    Untargeted(Untargeted),
}

impl From<Passive> for Module {
    fn from(m: Passive) -> Self {
        Self::Passive(m)
    }
}

impl From<Targeted> for Module {
    fn from(m: Targeted) -> Self {
        Self::Targeted(m)
    }
}

impl From<Untargeted> for Module {
    fn from(m: Untargeted) -> Self {
        Self::Untargeted(m)
    }
}
