mod entity;
pub mod instruction;
mod log;
mod round;
mod sites;

pub use entity::{Entity, EntityAsteroid};
pub use log::{Actor, Log};
pub use round::{advance, Output};
pub use sites::{Site, SitesNearPlanet};

#[cfg(feature = "typescript")]
ts_rs::export! {
    Actor,
    Log,
    SitesNearPlanet,
    Site => "typescript/generated-site.ts"
}
