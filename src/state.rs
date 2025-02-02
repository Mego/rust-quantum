use std::{
    cell::{Cell, Ref, RefCell},
    collections::{HashMap, HashSet},
    iter::repeat_n,
    rc::Rc,
};

use exhaust::Exhaust;

use itertools::Itertools;
use nalgebra::{
    ComplexField, DMatrix, DVector, Matrix2, SMatrix, SVector, SVectorView, Vector2, convert,
    dmatrix, dvector, try_convert,
};
use rand::{Rng, rng};
use rand_distr::{Distribution, weighted::WeightedIndex};

use crate::{
    qubit::Qubit,
    types::{Complex64, Measurement, Operation},
};

#[derive(Debug, Default)]
pub struct State {
    qubits: RefCell<DVector<Complex64>>,
    size: Cell<usize>,
}

impl State {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
            ..Default::default()
        })
    }

    pub fn qubit(self: Rc<Self>) -> Qubit {
        let idx = self.size.get();
        let q = Qubit::new(idx, self.clone());
        self.qubits.replace_with(|old| {
            old.kronecker(&Vector2::from_column_slice(&[
                Complex64::ZERO,
                Complex64::ONE,
            ]))
        });
        self.size.set(idx + 1);
        q
    }

    pub fn apply_operation<T: Operation>(self: Rc<Self>, op: &T, idx: usize) {
        let op_matrix = (0..self.size.get()).fold(dmatrix![], |m, i| {
            if i == idx {
                m.kronecker(op.matrix())
            } else {
                m.kronecker(&Matrix2::identity())
            }
        });
        self.qubits.replace_with(|old| op_matrix * &*old);
    }

    pub fn measure_all_z(self: Rc<Self>) -> Vec<Measurement> {
        let distr =
            WeightedIndex::new(self.qubits.borrow().iter().map(|c| c.norm_sqr() as f64)).unwrap();
        let mut options =
            repeat_n(Measurement::exhaust(), self.size.get()).multi_cartesian_product();
        let idx = distr.sample(&mut rng());
        self.qubits.replace_with(|old| {
            let mut m = DVector::zeros(old.nrows());
            m[2usize.pow(idx as u32)] = Complex64::ONE;
            m
        });
        options.nth(idx).unwrap()
    }

    pub fn measure_one_z(self: Rc<Self>, idx: usize) -> Measurement {
        let mut idxs = HashSet::new();
        let zero_prob: f64 = self
            .qubits
            .borrow()
            .iter()
            .enumerate()
            .map(|(i, c)| {
                if i & (1 << idx) == 0 {
                    idxs.insert(idx);
                    c.norm_sqr() as f64
                } else {
                    0f64
                }
            })
            .sum();
        let res = rng().random_bool(zero_prob);
        let mut qubits = self.qubits.borrow_mut();
        if res {
            for i in idxs {
                qubits[i] = Complex64::ZERO;
            }
            qubits.normalize_mut();
            return Measurement::ZERO;
        } else {
            for i in 0..qubits.len() {
                if !idxs.contains(&i) {
                    qubits[i] = Complex64::ZERO;
                }
            }
            qubits.normalize_mut();
            return Measurement::ONE;
        }
    }
}
