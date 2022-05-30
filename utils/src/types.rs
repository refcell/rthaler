use ark_poly::polynomial::multivariate::{SparsePolynomial, SparseTerm};
use ark_poly::polynomial::univariate::SparsePolynomial as UniSparsePolynomial;

/// A scalar field type.
pub type ScalarField = ark_bls12_381::Fr;

/// A multivariate polynomial over a scalar field and sparse term.
pub type MultiPoly = SparsePolynomial<ScalarField, SparseTerm>;

/// A univariate polynomial over a scalar field.
pub type UniPoly = UniSparsePolynomial<ScalarField>;
