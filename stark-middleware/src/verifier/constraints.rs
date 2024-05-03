use itertools::Itertools;
use p3_air::Air;
use p3_commit::PolynomialSpace;
use p3_field::{AbstractExtensionField, AbstractField, Field};
use p3_matrix::dense::RowMajorMatrixView;
use p3_matrix::stack::VerticalPair;
use p3_uni_stark::Domain;
use p3_uni_stark::StarkGenericConfig;
use p3_uni_stark::Val;
use tracing::instrument;

use crate::air_builders::verifier::VerifierConstraintFolder;
use crate::prover::opener::SingleAirOpenedValues;

use super::error::VerificationError;

#[instrument(skip_all)]
pub fn verify_single_air_constraints<SC, A>(
    air: &A,
    opened_values: &SingleAirOpenedValues<SC::Challenge>,
    // cumulative_sum: SC::Challenge,
    main_domain: Domain<SC>,
    qc_domains: &[Domain<SC>],
    zeta: SC::Challenge,
    alpha: SC::Challenge,
    // permutation_challenges: &[SC::Challenge],
    public_values: &[Val<SC>],
) -> Result<(), VerificationError>
where
    SC: StarkGenericConfig,
    A: for<'b> Air<VerifierConstraintFolder<'b, SC>> + ?Sized,
{
    let zps = qc_domains
        .iter()
        .enumerate()
        .map(|(i, domain)| {
            qc_domains
                .iter()
                .enumerate()
                .filter(|(j, _)| *j != i)
                .map(|(_, other_domain)| {
                    other_domain.zp_at_point(zeta)
                        * other_domain.zp_at_point(domain.first_point()).inverse()
                })
                .product::<SC::Challenge>()
        })
        .collect_vec();

    let quotient = opened_values
        .quotient_chunks
        .iter()
        .enumerate()
        .map(|(ch_i, ch)| {
            ch.iter()
                .enumerate()
                .map(|(e_i, &c)| zps[ch_i] * SC::Challenge::monomial(e_i) * c)
                .sum::<SC::Challenge>()
        })
        .sum::<SC::Challenge>();

    let unflatten = |v: &[SC::Challenge]| {
        v.chunks_exact(SC::Challenge::D)
            .map(|chunk| {
                chunk
                    .iter()
                    .enumerate()
                    .map(|(e_i, &c)| SC::Challenge::monomial(e_i) * c)
                    .sum()
            })
            .collect::<Vec<SC::Challenge>>()
    };

    let sels = main_domain.selectors_at_point(zeta);

    let main = VerticalPair::new(
        RowMajorMatrixView::new_row(&opened_values.trace.local),
        RowMajorMatrixView::new_row(&opened_values.trace.next),
    );

    let mut folder: VerifierConstraintFolder<'_, SC> = VerifierConstraintFolder {
        main,
        public_values,
        is_first_row: sels.is_first_row,
        is_last_row: sels.is_last_row,
        is_transition: sels.is_transition,
        alpha,
        accumulator: SC::Challenge::zero(),
    };
    air.eval(&mut folder);

    let folded_constraints = folder.accumulator;
    // Finally, check that
    //     folded_constraints(zeta) / Z_H(zeta) = quotient(zeta)
    if folded_constraints * sels.inv_zeroifier != quotient {
        return Err(VerificationError::OodEvaluationMismatch);
    }

    Ok(())
}
