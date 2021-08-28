#![deny(unsafe_code)]
#![allow(clippy::implicit_hasher)]

mod serde_helper;

/// Something floating around in space.
pub mod entity;
/// Static data which is read-only for everyone.
/// It only changes on game-updates.
pub mod fixed;
pub mod market;
pub mod player;
pub mod ship;
pub mod site;
pub mod storage;

/// Data the frontend reads ands writes.
pub mod frontrw;

#[cfg(test)]
mod test_helper;
