#[macro_use]
extern crate lazy_static;

use ark_poly::{polynomial::multivariate::{SparsePolynomial, SparseTerm, Term}, MVPolynomial};
use utils::types::*;

use sumcheck::prover::*;

lazy_static! {
	// g = 2(x_1)^3 + (x_1)(x_3) + (x_2)(x_3)
	static ref G_0: sumcheck::MultiPoly = SparsePolynomial::from_coefficients_vec(
		3,
		vec![
			(2u32.into(), SparseTerm::new(vec![(0, 3)])),
			(1u32.into(), SparseTerm::new(vec![(0, 1), (2, 1)])),
			(1u32.into(), SparseTerm::new(vec![(1, 1), (2, 1)])),
		],
	);
	static ref G_0_SUM: ScalarField = sumcheck::Prover::new(&G_0).slow_sum_g();
	// Test with a larger g
	static ref G_1: sumcheck::MultiPoly = SparsePolynomial::from_coefficients_vec(
		4,
		vec![
			(2u32.into(), SparseTerm::new(vec![(0, 3)])),
			(1u32.into(), SparseTerm::new(vec![(0, 1), (2, 1)])),
			(1u32.into(), SparseTerm::new(vec![(1, 1), (2, 1)])),
			(1u32.into(), SparseTerm::new(vec![(3, 1), (2, 1)])),
		],
	);
	static ref G_1_SUM: ScalarField = sumcheck::Prover::new(&G_1).slow_sum_g();
}

#[test]
fn instantiates() {
  let p: Prover = Prover::new();

}