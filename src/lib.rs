#![forbid(unsafe_code)]
#![allow(clippy::implicit_hasher)]

mod serde_helper;

/// Something floating around in space.
pub mod entity;
/// Static data which is read-only for everyone.
/// It only changes on game-updates.
pub mod fixed;
pub mod player;
pub mod ship;
pub mod site;
pub mod station;
pub mod storage;

#[cfg(feature = "market")]
pub mod market;

#[cfg(test)]
mod test_helper;
