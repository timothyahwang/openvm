use p3_air::{Air, BaseAir};
use p3_uni_stark::{StarkGenericConfig, Val};
use serde::{Deserialize, Serialize};

use crate::{
    air_builders::{symbolic::SymbolicAirBuilder, verifier::VerifierConstraintFolder},
    config::Com,
};

/// Verifier data for multi-matrix trace commitments.
/// The data is for the traces committed into a single commitment.
#[derive(Serialize, Deserialize)]
pub struct VerifierTraceData<SC: StarkGenericConfig> {
    /// The heights of the trace matrices.
    /// Equivalently, the number of rows for each matrix.
    /// Equivalently, the domain size for each trace.
    pub degrees: Vec<usize>,
    /// Commitment to the trace matrices.
    pub commit: Com<SC>,
}

/// Verifier data for multi-matrix quotient polynomial commitment.
/// Quotient polynomials for multiple AIRs that share a multi-matrix trace commitment
/// are committed together into a single commitment.
#[derive(Serialize, Deserialize)]
pub struct VerifierQuotientData<SC: StarkGenericConfig> {
    /// Number of quotient chunks that were committed.
    pub quotient_degrees: Vec<usize>,
    /// Quotient commitment
    pub commit: Com<SC>,
}

/// AIR trait for verifier use.
pub trait VerifierAir<SC: StarkGenericConfig>:
    for<'a> Air<VerifierConstraintFolder<'a, SC>> + Air<SymbolicAirBuilder<Val<SC>>>
{
}

impl<SC: StarkGenericConfig, T> VerifierAir<SC> for T where
    T: for<'a> Air<VerifierConstraintFolder<'a, SC>> + Air<SymbolicAirBuilder<Val<SC>>>
{
}
