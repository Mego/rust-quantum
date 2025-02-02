use bounded_integer::BoundedU8;
use itertools::Itertools;
use nalgebra::{DMatrix, DVector, RowDVector, SMatrix, SVector};

use crate::types::{Bit, Complex64};

pub(crate) fn permutation_smatrix<const N: usize>(
    permutation: [usize; N],
) -> SMatrix<Complex64, N, N> {
    let ii = SMatrix::<Complex64, N, N>::identity();
    SMatrix::from_rows(
        &permutation
            .into_iter()
            .map(|p| ii.row(p))
            .collect_array::<N>()
            .unwrap(),
    )
}

pub(crate) fn permutation_dmatrix(permutation: &[usize]) -> DMatrix<Complex64> {
    let ii = DMatrix::<Complex64>::identity(permutation.len(), permutation.len());
    DMatrix::from_rows(&permutation.into_iter().map(|&p| ii.row(p)).collect_vec())
}

pub(crate) fn is_unitary<const N: usize>(m: &SMatrix<Complex64, N, N>) -> bool {
    (m * m.adjoint()).is_identity(0.into())
}

pub(crate) fn ket(values: &[Bit]) -> DVector<Complex64> {
    DVector::from_iterator(
        values.len() * 2,
        values.iter().flat_map(|&v| {
            if v == 0 {
                [Complex64::ONE, Complex64::ZERO]
            } else {
                [Complex64::ZERO, Complex64::ONE]
            }
        }),
    )
}

pub(crate) fn bra(values: &[Bit]) -> RowDVector<Complex64> {
    RowDVector::from_iterator(
        values.len() * 2,
        values.iter().flat_map(|&v| {
            if v == 0 {
                [Complex64::ONE, Complex64::ZERO]
            } else {
                [Complex64::ZERO, Complex64::ONE]
            }
        }),
    )
}
