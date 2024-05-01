use p3_air::Air;
use p3_matrix::dense::RowMajorMatrix;
use p3_uni_stark::{Domain, StarkGenericConfig, Val};
use serde::{Deserialize, Serialize};

use crate::{
    air_builders::{prover::ProverConstraintFolder, symbolic::SymbolicAirBuilder},
    config::{Com, PcsProof, PcsProverData},
};

use super::opener::OpenedValues;

/// Prover data for multi-matrix trace commitments.
/// The data is for the traces committed into a single commitment.
///
/// This data can be cached and attached to other multi-matrix traces.
pub struct ProverTraceData<SC: StarkGenericConfig> {
    /// Trace matrices, possibly of different heights.
    /// We store the domain each trace was committed with respect to.
    // Memory optimization? PCS ProverData should be able to recover the domain.
    pub traces_with_domains: Vec<(Domain<SC>, RowMajorMatrix<Val<SC>>)>,
    /// Commitment to the trace matrices.
    pub commit: Com<SC>,
    /// Prover data, such as a Merkle tree, for the trace commitment.
    pub data: PcsProverData<SC>,
}

/// Prover data for multiple AIRs that share a multi-matrix trace commitment.
/// Each AIR owns a separate trace matrix of a different height, but these
/// trace matrices have been committed together using the PCS.
///
/// This struct contains references to the AIRs themselves, which hold the constraint
/// information. The AIRs must support the same [AirBuilder].
///
/// We use dynamic dispatch here for the extra flexibility. The overhead is small since
/// the number of AIRs will not be more than 100 and the prover only needs to
/// dispatch once per AIR.
///
/// The ordering of `trace_data.traces` and `airs` must match.
pub struct ProvenMultiMatrixAirTrace<'a, SC: StarkGenericConfig> {
    /// Proven trace data.
    pub trace_data: &'a ProverTraceData<SC>,
    /// The AIRs that share the trace commitment.
    pub airs: Vec<&'a dyn ProverAir<SC>>,
}

impl<'a, SC: StarkGenericConfig> Clone for ProvenMultiMatrixAirTrace<'a, SC> {
    fn clone(&self) -> Self {
        Self {
            trace_data: self.trace_data,
            airs: self.airs.clone(),
        }
    }
}

/// Prover data for multi-matrix quotient polynomial commitment.
/// Quotient polynomials for multiple AIRs that share a multi-matrix trace commitment
/// are committed together into a single commitment.
pub struct ProverQuotientData<SC: StarkGenericConfig> {
    /// Number of quotient chunks that were committed.
    pub quotient_degrees: Vec<usize>,
    /// Quotient commitment
    pub commit: Com<SC>,
    /// Prover data for the quotient commitment
    pub data: PcsProverData<SC>,
}

/// Prover data for multiple AIRs that share a single trace commitment
/// and a single quotient commitment.
pub struct ProvenDataBeforeOpening<'a, SC: StarkGenericConfig> {
    pub trace: &'a ProverTraceData<SC>,
    pub quotient: &'a ProverQuotientData<SC>,
}

pub struct OpeningProofData<SC: StarkGenericConfig> {
    pub proof: PcsProof<SC>,
    pub values: OpenedValues<SC::Challenge>,
}

#[derive(Serialize, Deserialize)]
pub struct Commitments<Com> {
    pub main_trace: Com,
    // pub perm_trace: Com,
    pub quotient: Com,
}

pub struct PartitionedProof<SC: StarkGenericConfig> {
    /// Commitments separated by partition
    // TODO: I think quotient commitment should be shared
    pub commitments: Vec<Commitments<Com<SC>>>,
    pub opening_proofs: Vec<OpeningProofData<SC>>,
}

pub trait ProverAir<SC: StarkGenericConfig>:
    for<'a> Air<ProverConstraintFolder<'a, SC>> + Air<SymbolicAirBuilder<Val<SC>>>
{
}

impl<SC: StarkGenericConfig, T> ProverAir<SC> for T where
    T: for<'a> Air<ProverConstraintFolder<'a, SC>> + Air<SymbolicAirBuilder<Val<SC>>>
{
}
