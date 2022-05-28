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

  /// Simulates a Prover calculating the proof.
  pub fn simulate(&mut self, r: Option<ScalarField>) -> UniPoly {
    r.is_some().and_then(|r| self.r_vec.push(r.unwrap()));
    let v = self.g.num_vars() - self.r_vec.len();
    (0..(2u32.pow(v as u32 - 1))).fold(
      UniPoly::from_coefficients_vec(vec![(0, 0u32.into())]),
      |sum, n| sum + self.evaluate_gj(n_to_vec(n as usize, v)),
    )
  }

  // Evaluates gj over a vector permutation of points, folding all evaluated terms together into one univariate polynomial
	pub fn evaluate_gj(&self, points: Vec<ScalarField>) -> UniPoly {
		cfg_into_iter!(self.g.terms()).fold(
			UniPoly::from_coefficients_vec(vec![]),
			|sum, (coeff, term)| {
				let (coeff_eval, fixed_term) = self.evaluate_term(&term, &points);
				let curr = match fixed_term {
					None => UniPoly::from_coefficients_vec(vec![(0, *coeff * coeff_eval)]),
					_ => UniPoly::from_coefficients_vec(vec![(
						fixed_term.unwrap().degree(),
						*coeff * coeff_eval,
					)]),
				};
				curr + sum
			},
		)
	}

  // Evaluates a term with a fixed univar, returning (new coefficent, fixed term)
	pub fn evaluate_term(
		&self,
		term: &SparseTerm,
		point: &Vec<ScalarField>,
	) -> (ScalarField, Option<SparseTerm>) {
		let mut fixed_term: Option<SparseTerm> = None;
		let coeff: ScalarField =
			cfg_into_iter!(term).fold(1u32.into(), |product, (var, power)| match *var {
				j if j == self.r_vec.len() => {
					fixed_term = Some(SparseTerm::new(vec![(j, *power)]));
					product
				}
				j if j < self.r_vec.len() => self.r_vec[j].pow(&[*power as u64]) * product,
				_ => point[*var - self.r_vec.len()].pow(&[*power as u64]) * product,
			});
		(coeff, fixed_term)
	}

	// Sum all evaluations of polynomial `g` over boolean hypercube
	pub fn slow_sum_g(&self) -> ScalarField {
		let v = self.g.num_vars();
		let n = 2u32.pow(v as u32);
		(0..n)
			.map(|n| self.g.evaluate(&n_to_vec(n as usize, v)))
			.sum()
	}
}
