//! Middleware for proving and verifying mixed-matrix STARKs with univariate polynomial commitment scheme.

/// AIR builders for prover and verifier, including support for cross-matrix permutation arguments.
pub mod air_builders;
pub mod chip;
/// Helper types associated to generic STARK config.
pub mod config;
pub mod interaction;
pub mod prover;
pub mod util;
