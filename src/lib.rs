//! # silo-core
//!
//! **Core math silo for the SuperInstance ternary fleet.**
//!
//! Separates pure Z₃ computation (the Silo) from the Pincher reflex runtime.
//! All ternary math crates in the fleet depend on `silo-core` → `ternary-types`,
//! forming a clean dependency hierarchy:
//!
//! ```text
//! ┌─────────────────────────────────────────────┐
//! │               Pincher (runtime)              │
//! │  Vector DB, reflexes, agents, persistence   │
//! ├─────────────────────────────────────────────┤
//! │              ternary-math-crates             │
//! │  (matmul, conv, fuse, hmm, knn, prune, …)  │
//! ├─────────────────── SILO ────────────────────┤
//! │                silo-core                     │
//! │   Core algebra, geometry, analysis on Z₃    │
//! ├─────────────────────────────────────────────┤
//! │              ternary-types                   │
//! │      Ternary enum, TritVector, Matrix       │
//! └─────────────────────────────────────────────┘
//! ```
//!
//! ## Silo vs Pincher
//!
//! | Layer | Role | Dependencies |
//! |-------|------|--------------|
//! | **Silo** | Pure math on Z₃ | `ternary-types` only |
//! | **Pincher** | Runtime, reflexes, persistence | `silo-core` + `ternary-types` |

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod algebra;
pub mod analysis;
pub mod geometry;

// Re-export ternary-types for convenience
pub use ternary_types::{Ternary, TritVector, TernaryMatrix, PackedTrits};

/// Short aliases for Ternary variants.
pub mod trits {
    pub use ternary_types::Ternary::{Negative, Neutral, Positive};
}
