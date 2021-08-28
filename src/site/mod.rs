mod entity;
pub mod entity_frontread;
pub mod instruction;
mod log;
mod sites;

pub use entity::{Entity, EntityAsteroid};
pub use log::{Actor, Log};
pub use sites::{Site, SitesNearPlanet};

#[cfg(feature = "site_round")]
mod round;
#[cfg(feature = "site_round")]
pub use round::{advance, Output};

#[cfg(feature = "typescript")]
ts_rs::export! {
    Actor,
    Log,
    SitesNearPlanet,
    Site => "typescript/generated-site.ts"
}
