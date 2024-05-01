use itertools::{izip, Itertools};
use p3_commit::{Pcs, PolynomialSpace};
use p3_uni_stark::StarkGenericConfig;
use serde::{Deserialize, Serialize};

use super::types::{OpeningProofData, ProvenDataBeforeOpening};

pub struct OpeningProver<'pcs, SC: StarkGenericConfig> {
    pcs: &'pcs SC::Pcs,
    zeta: SC::Challenge,
}

impl<'pcs, SC: StarkGenericConfig> OpeningProver<'pcs, SC> {
    pub fn new(pcs: &'pcs SC::Pcs, zeta: SC::Challenge) -> Self {
        Self { pcs, zeta }
    }

    pub fn open(
        &self,
        challenger: &mut SC::Challenger,
        data: ProvenDataBeforeOpening<SC>,
    ) -> OpeningProofData<SC> {
        let zeta = self.zeta;
        let trace_data = &data.trace.data;
        let trace_opening_points = data
            .trace
            .traces_with_domains
            .iter()
            .map(|(domain, _)| vec![zeta, domain.next_point(zeta).unwrap()])
            .collect_vec();
        let quotient_data = &data.quotient.data;
        // open every chunk at zeta
        let quotient_degrees = &data.quotient.quotient_degrees;
        let num_chunks: usize = quotient_degrees.iter().sum();
        let quotient_opening_points = vec![vec![zeta]; num_chunks];

        let (opening_values, opening_proof) = self.pcs.open(
            vec![
                // (&preprocessed_data, preprocessed_opening_points),
                (trace_data, trace_opening_points),
                // (&perm_data, main_opening_points),
                (quotient_data, quotient_opening_points),
            ],
            challenger,
        );

        // Collect the opened values for each chip.
        let [trace_openings, mut quotient_openings] = opening_values
            .try_into()
            .expect("Should have 2 rounds of openings");

        let trace_openings = trace_openings.into_iter().map(|op| {
            let [local, next] = op.try_into().expect("Should have 2 openings");
            AdjacentOpenedValues { local, next }
        });

        // Unflatten quotient openings
        let quotient_openings = quotient_degrees.iter().map(|&chunk_size| {
            quotient_openings
                .drain(..chunk_size)
                .map(|mut op| {
                    op.pop()
                        .expect("quotient chunk should be opened at 1 point")
                })
                .collect_vec()
        });

        let per_air = izip!(trace_openings, quotient_openings)
            .map(|(trace, quotient_chunks)| SingleAirOpenedValues {
                trace,
                quotient_chunks,
            })
            .collect::<Vec<_>>();
        OpeningProofData {
            proof: opening_proof,
            values: OpenedValues { per_air },
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct OpenedValues<Challenge> {
    /// Opened values grouped by AIR
    pub per_air: Vec<SingleAirOpenedValues<Challenge>>,
}

#[derive(Serialize, Deserialize)]
pub struct SingleAirOpenedValues<Challenge> {
    pub trace: AdjacentOpenedValues<Challenge>,
    pub quotient_chunks: Vec<Vec<Challenge>>,
}

#[derive(Serialize, Deserialize)]
pub struct AdjacentOpenedValues<Challenge> {
    pub local: Vec<Challenge>,
    pub next: Vec<Challenge>,
}
