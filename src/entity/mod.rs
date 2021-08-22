mod collateral;

pub use collateral::{Collateral, Health};

#[cfg(feature = "typescript")]
ts_rs::export! {
    Collateral => "entity-collateral.ts",
    Health => "entity-health.ts",
}
