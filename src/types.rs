use bounded_integer::BoundedU8;
use exhaust::Exhaust;
use nalgebra::{Complex, SMatrix, SVectorView, SVectorViewMut};

use crate::{algebra::is_unitary, qubit::Qubit};

pub type Complex64 = Complex<f64>;

pub type Bit = BoundedU8<0, 1>;

#[derive(Debug, Clone, Copy, Exhaust)]
pub enum Measurement {
    ZERO = 0,
    ONE = 1,
}

impl From<bool> for Measurement {
    fn from(value: bool) -> Self {
        if value { Self::ONE } else { Self::ZERO }
    }
}

pub trait Operation {
    fn matrix(&self) -> &SMatrix<Complex64, 2, 2>;
    fn adjoint(&self) -> Self;
    fn apply(&self, qubit: &Qubit);

    fn then<T: Operation>(&self, other: &T) -> MatrixOperation {
        MatrixOperation(other.matrix() * self.matrix())
    }
}

pub(crate) struct MatrixOperation(SMatrix<Complex64, 2, 2>);

impl MatrixOperation {
    pub(crate) fn new(m: SMatrix<Complex64, 2, 2>) -> Self {
        assert!(is_unitary(&m));
        Self(m)
    }

    pub(crate) fn identity() -> Self {
        Self(SMatrix::<Complex64, 2, 2>::identity())
    }
}

impl Operation for MatrixOperation {
    fn matrix(&self) -> &SMatrix<Complex64, 2, 2> {
        &self.0
    }

    fn adjoint(&self) -> Self {
        Self(self.0.adjoint())
    }

    fn apply(&self, qubit: &Qubit) {
        qubit.apply_operation(self);
    }
}
