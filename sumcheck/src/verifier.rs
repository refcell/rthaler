use ark_bls12_381::Fr as ScalarField;

use ark_ff::Field;
use ark_poly::polynomial::multivariate::{SparsePolynomial, SparseTerm, Term};
use ark_poly::polynomial::univariate::SparsePolynomial as UniSparsePolynomial;
use ark_poly::polynomial::{MVPolynomial, Polynomial};
use ark_std::cfg_into_iter;
use rand::Rng;

use crate::common::*;

/// ## Verifier
///
/// The verifier is responsible for verifing the proof of summation over the polynomial.
#[derive(Debug, Clone)]
pub struct Verifier {
	pub g: MultiPoly,
}

impl Verifier {
  /// Instantiates a new Verifier instance.
	pub fn new(g: &MultiPoly) -> Self {
		Prover {
			g: g.clone(),
		}
	}

  /// #### `rnd` [Public Associated Function]
  ///
  /// [Public Associated Function]
  /// Generates a random scalar.
  pub fn rnd() -> Option<ScalarField> {
    let mut rng: ThreadRng = rand::thread_rng();
    let r: ScalarField = rng.gen();
    Some(r)
  }

  /// #### `degrees` [Public Associated Function]
  ///
  /// Generates a lookup table for max degrees of a polynomial.
  pub fn degrees(g: &MultiPoly) -> Vec<usize> {
    let mut lookup: Vec<usize> = vec![0; g.num_vars()];
    cfg_into_iter!(g.terms()).for_each(|(_, term)| {
      cfg_into_iter!(term).for_each(|(var, power)| {
        if *power > lookup[*var] {
          lookup[*var] = *power
        }
      });
    });
    lookup
  }

  /// #### `verify`
  ///
  /// Verifies prover's claim
  /// Presented pedantically:
  pub fn verify(self, claim: ScalarField) -> bool {
    // 1st round
    // let mut p = Prover::new(g);
    // let mut gi = p.gen_uni_polynomial(None);
    // let mut expected_c = gi.evaluate(&0u32.into()) + gi.evaluate(&1u32.into());
    // assert_eq!(c_1, expected_c);

    // Validate Polynomial degrees
    let lookup_degree: Vec<usize> = Verifier::degrees(&self.g);
    assert!(gi.degree() <= lookup_degree[0]);

    // Interactive 
    let mut expected_claim;
    for j in 1..self.g.num_vars() {
      let r: Option<ScalarField> = Verifier::rnd();
      expected_claim = gi.evaluate(&r.unwrap());
      gi = p.gen_uni_polynomial(r);
      let new_c = gi.evaluate(&0u32.into()) + gi.evaluate(&1u32.into());
      assert_eq!(expected_claim, new_c);
      assert!(gi.degree() <= lookup_degree[j]);
    }

    // final round
    let r = Verifier::rnd();
    expected_claim = gi.evaluate(&r.unwrap());
    p.r_vec.push(r.unwrap());
    let new_c = p.g.evaluate(&p.r_vec);
    assert_eq!(expected_claim, new_c);
    true
  }

  pub fn slow_verify(g: &MultiPoly, c_1: ScalarField) -> bool {
    let p = Prover::new(g);
    let manual_sum = p.slow_sum_g();
    manual_sum == c_1
  }
}