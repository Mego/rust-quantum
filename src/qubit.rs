use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use nalgebra::{DVector, DVectorView, Dyn, SVector, SVectorView, SVectorViewMut};

use crate::{
    state::State,
    types::{Complex64, Operation},
};

#[derive(Debug)]
pub struct Qubit {
    idx: usize,
    state: Rc<State>,
}

impl Qubit {
    pub(crate) fn new(idx: usize, state: Rc<State>) -> Self {
        Self { idx, state }
    }

    pub(crate) fn state(&self) -> Rc<State> {
        self.state.clone()
    }

    pub fn apply_operation<T: Operation>(&self, op: &T) {
        self.state().apply_operation(op, self.idx);
    }
}
