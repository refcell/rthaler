use ark_bls12_381::Fr as ScalarField;

use ark_ff::Field;
use ark_poly::polynomial::multivariate::{SparsePolynomial, SparseTerm, Term};
use ark_poly::polynomial::univariate::SparsePolynomial as UniSparsePolynomial;
use ark_poly::polynomial::{MVPolynomial, Polynomial};
use ark_std::cfg_into_iter;
use rand::Rng;

use crate::common::*;

/// ## Prover
///
/// The prover is responsible for generating the proof of summation over the polynomial.
#[derive(Debug, Clone)]
pub struct Prover {
	pub g: MultiPoly,
	pub r_vec: Vec<ScalarField>,
}

impl Prover {
  /// Instantiates a new Prover instance.
	pub fn new(g: &MultiPoly) -> Self {
		Prover {
			g: g.clone(),
			r_vec: vec![],
		}
	}


}
