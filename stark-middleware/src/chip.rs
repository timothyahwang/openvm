use p3_air::{Air, AirBuilder};
use p3_field::Field;
use p3_matrix::dense::RowMajorMatrix;

use crate::interaction::{Interaction, InteractionType};

pub trait Chip<F: Field> {
    fn generate_trace(&self) -> RowMajorMatrix<F>;

    fn sends(&self) -> Vec<Interaction<F>> {
        vec![]
    }

    fn receives(&self) -> Vec<Interaction<F>> {
        vec![]
    }

    fn all_interactions(&self) -> Vec<(Interaction<F>, InteractionType)> {
        let mut interactions: Vec<(Interaction<F>, InteractionType)> = vec![];
        interactions.extend(self.sends().into_iter().map(|i| (i, InteractionType::Send)));
        interactions.extend(
            self.receives()
                .into_iter()
                .map(|i| (i, InteractionType::Receive)),
        );
        interactions
    }
}

/// An interactive AIR is an AIR that can specify channels for sending and receiving data
/// to other AIRs. The AIR does not specify the constraints for the channels itself.
/// These constraints are defined by the prover elsewhere
/// (typically using a permutation argument).
// pub type InteractiveAir<AB: AirBuilder> = Chip<AB::F> + Air<AB>;
