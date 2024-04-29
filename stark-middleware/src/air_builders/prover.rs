// Folder: Folding builder
use p3_air::{AirBuilder, AirBuilderWithPublicValues, ExtensionBuilder};
use p3_field::AbstractField;
use p3_matrix::dense::RowMajorMatrixView;
use p3_matrix::stack::VerticalPair;
use p3_uni_stark::{PackedChallenge, PackedVal, StarkGenericConfig, Val};

/// A folder for prover constraints.
pub struct ProverConstraintFolder<'a, SC: StarkGenericConfig> {
    // pub preprocessed:
    //     VerticalPair<RowMajorMatrixView<'a, PackedVal<SC>>, RowMajorMatrixView<'a, PackedVal<SC>>>,
    pub main:
        VerticalPair<RowMajorMatrixView<'a, PackedVal<SC>>, RowMajorMatrixView<'a, PackedVal<SC>>>,
    // pub perm: VerticalPair<
    //     RowMajorMatrixView<'a, PackedChallenge<SC>>,
    //     RowMajorMatrixView<'a, PackedChallenge<SC>>,
    // >,
    // pub perm_challenges: &'a [PackedChallenge<SC>],
    // pub cumulative_sum: SC::Challenge,
    pub is_first_row: PackedVal<SC>,
    pub is_last_row: PackedVal<SC>,
    pub is_transition: PackedVal<SC>,
    pub alpha: SC::Challenge,
    pub accumulator: PackedChallenge<SC>,
    pub public_values: &'a [Val<SC>],
}

impl<'a, SC> AirBuilder for ProverConstraintFolder<'a, SC>
where
    SC: StarkGenericConfig,
{
    type F = Val<SC>;
    type Expr = PackedVal<SC>;
    type Var = PackedVal<SC>;
    type M =
        VerticalPair<RowMajorMatrixView<'a, PackedVal<SC>>, RowMajorMatrixView<'a, PackedVal<SC>>>;

    fn main(&self) -> Self::M {
        self.main
    }

    fn is_first_row(&self) -> Self::Expr {
        self.is_first_row
    }

    fn is_last_row(&self) -> Self::Expr {
        self.is_last_row
    }

    fn is_transition_window(&self, size: usize) -> Self::Expr {
        if size == 2 {
            self.is_transition
        } else {
            panic!("only supports a window size of 2")
        }
    }

    fn assert_zero<I: Into<Self::Expr>>(&mut self, x: I) {
        let x: PackedVal<SC> = x.into();
        self.accumulator *= PackedChallenge::<SC>::from_f(self.alpha);
        self.accumulator += x;
    }
}

// impl<'a, SC> PairBuilder for ProverConstraintFolder<'a, SC>
// where
//     SC: StarkGenericConfig,
// {
//     fn preprocessed(&self) -> Self::M {
//         // TODO: Avoid clone?
//         self.preprocessed.clone()
//     }
// }

impl<'a, SC> ExtensionBuilder for ProverConstraintFolder<'a, SC>
where
    SC: StarkGenericConfig,
{
    type EF = SC::Challenge;
    type ExprEF = PackedChallenge<SC>;
    type VarEF = PackedChallenge<SC>;

    fn assert_zero_ext<I>(&mut self, x: I)
    where
        I: Into<Self::ExprEF>,
    {
        let x: PackedChallenge<SC> = x.into();
        self.accumulator *= PackedChallenge::<SC>::from_f(self.alpha);
        self.accumulator += x;
    }
}

// impl<'a, SC> PermutationAirBuilder for ProverConstraintFolder<'a, SC>
// where
//     SC: StarkGenericConfig,
// {
//     type MP = RowMajorMatrix<PackedChallenge<SC>>;

//     type RandomVar = PackedChallenge<SC>;

//     fn permutation(&self) -> Self::MP {
//         self.perm.clone()
//     }

//     fn permutation_randomness(&self) -> &[Self::RandomVar] {
//         self.perm_challenges
//     }
// }

impl<'a, SC: StarkGenericConfig> AirBuilderWithPublicValues for ProverConstraintFolder<'a, SC> {
    type PublicVar = Self::F;

    fn public_values(&self) -> &[Self::F] {
        self.public_values
    }
}
