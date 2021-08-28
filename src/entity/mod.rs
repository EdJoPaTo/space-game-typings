mod collateral;

pub use collateral::{Collateral, Health};

#[cfg(feature = "typescript")]
ts_rs::export! {
    Collateral, Health => "typescript/generated-collateral.ts",
}
