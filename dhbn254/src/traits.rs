//! Traits
//!
//! Traits for dhbn254 that must be implemented.


/// Modular Exponentiation
///
/// Generates the modular exponentiation of base `self` to the power of `exponent` divided by modulus `modulus`.
/// The result is of type `Self` (the type on which the trait is implemented).
pub trait ModularExponentiation {
  fn modexp(&self, power: Self, modulus: Self) -> Self;
}
