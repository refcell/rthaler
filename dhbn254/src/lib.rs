#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]
#![deny(missing_docs)]

/// Traits
pub mod traits;

/// Common Types
pub mod types;

/// EphemeralSecret
pub mod ephemeral;

/// Prelude
pub mod prelude {
    // Re-export our common traits and types
    pub use crate::traits::*;
    pub use crate::types::*;

    // Re-export EphemeralSecret and associated traits
    pub use crate::ephemeral::*;
}

/// Actors Feature
#[cfg(feature = "actors")]
pub mod actors;
