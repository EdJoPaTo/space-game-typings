mod serde_helper;

/// Static data which is read-only for everyone.
/// It only changes on game-updates.
pub mod fixed;

/// Persistent data which is read/write by the backend.
pub mod persist;

/// Data which is available to read for the frontend.
/// This data is exported from persistant data.
pub mod frontread;

/// Data the frontend reads ands writes.
pub mod frontrw;

/// Ships that can be found in sites, warping or docked
pub mod ship;

#[cfg(test)]
mod test_helper;
