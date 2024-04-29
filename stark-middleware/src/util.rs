extern crate alloc;

use p3_air::Air;
use p3_field::Field;
use p3_uni_stark::{get_max_constraint_degree, SymbolicAirBuilder};
use p3_util::log2_ceil_usize;
use tracing::instrument;

#[instrument(name = "infer log of constraint degree", skip_all)]
pub fn get_log_quotient_degree<F, A>(air: &A, num_public_values: usize) -> usize
where
    F: Field,
    A: Air<SymbolicAirBuilder<F>> + ?Sized,
{
    // We pad to at least degree 2, since a quotient argument doesn't make sense with smaller degrees.
    let constraint_degree = get_max_constraint_degree(air, num_public_values).max(2);

    // The quotient's actual degree is approximately (max_constraint_degree - 1) n,
    // where subtracting 1 comes from division by the zerofier.
    // But we pad it to a power of two so that we can efficiently decompose the quotient.
    log2_ceil_usize(constraint_degree - 1)
}
