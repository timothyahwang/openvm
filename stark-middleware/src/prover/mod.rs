use itertools::Itertools;
use p3_challenger::{CanObserve, FieldChallenger};
use p3_maybe_rayon::prelude::*;
use p3_uni_stark::{get_log_quotient_degree, Domain, StarkGenericConfig, Val};

use crate::config::{Com, OpeningProof, PcsProverData};

use self::{
    committer::quotient::QuotientCommitter,
    opener::OpeningProver,
    types::{ProvenDataBeforeOpening, ProvenMultiMatrixAirTrace},
};

pub mod committer;
pub mod opener;
pub mod types;

/// Proves a partition of multi-matrix AIRs.
// TODO: Interactions between parts in partition.
pub struct PartitionProver<SC: StarkGenericConfig> {
    config: SC,
}

impl<SC: StarkGenericConfig> PartitionProver<SC> {
    /// Assumes the traces have been generated already.
    pub fn prove<'a>(
        &self,
        challenger: &mut SC::Challenger,
        // TODO: proving key,
        partition: Vec<ProvenMultiMatrixAirTrace<'a, SC>>,
        public_values: Vec<Vec<Val<SC>>>,
    ) where
        SC::Pcs: Sync,
        Domain<SC>: Send + Sync,
        PcsProverData<SC>: Send + Sync,
        Com<SC>: Send + Sync,
        SC::Challenge: Send + Sync,
        OpeningProof<SC>: Send + Sync,
    {
        let pcs = self.config.pcs();

        // TODO: preprocessed (aka proving key)

        // Challenger must observe all trace commitments
        for part in partition.iter() {
            challenger.observe(part.trace_data.commit.clone());
        }

        // TODO: permutation trace

        // Generate `alpha` challenge
        let alpha: SC::Challenge = challenger.sample_ext_element();

        let quotient_committer = QuotientCommitter::new(pcs, alpha);
        let quotient_data = partition
            .par_iter()
            .zip(public_values.par_iter())
            .map(|(part, public_values)| {
                let quotient_degrees = part
                    .airs
                    .iter()
                    .map(|&air| {
                        let d = get_log_quotient_degree::<Val<SC>, _>(air, public_values.len());
                        1 << d
                    })
                    .collect_vec();
                let qv = quotient_committer.compute_quotient_values(
                    part.clone(),
                    quotient_degrees,
                    public_values,
                );
                quotient_committer.commit(qv)
            })
            .collect::<Vec<_>>();

        // Observe all quotient commitments
        for q in quotient_data.iter() {
            challenger.observe(q.commit.clone());
        }

        let proven_partitions = partition
            .into_iter()
            .zip_eq(quotient_data.iter())
            .map(|(part, quotient)| ProvenDataBeforeOpening {
                trace: part.trace_data,
                quotient,
            })
            .collect::<Vec<_>>();

        // Draw `zeta` challenge
        let zeta: SC::Challenge = challenger.sample_ext_element();

        let opener = OpeningProver::new(pcs, zeta);
        let opening_data = proven_partitions
            .into_par_iter()
            .map(|part| opener.open(challenger, part))
            .collect::<Vec<_>>();
    }
}
