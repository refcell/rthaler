//! Ephemeral
//!
//! Ephemeral Secret with extensive trait implementations.

use std::convert::TryFrom;
use ark_ff::{PrimeField};
use rand_core::{CryptoRng, RngCore};
use serde::{Deserialize, Serialize};

use crate::types::*;
use crate::traits::*;

#[cfg(test)]
use proptest::{arbitrary::Arbitrary, array, collection, prelude::*};

/// A shared secret key that exists for a brief period of time.
#[cfg_attr(test, derive(Debug))]
pub struct EphemeralSecret(pub(crate) Scalar);

impl From<[u8; 32]> for EphemeralSecret {
    fn from(bytes: [u8; 32]) -> EphemeralSecret {
        Self(Scalar::from_be_bytes_mod_order(&bytes))
    }
}

impl From<[u8; 64]> for EphemeralSecret {
    fn from(bytes: [u8; 64]) -> EphemeralSecret {
        Self(Scalar::from_be_bytes_mod_order(&bytes))
    }
}

pub trait Random = RngCore + CryptoRng;

impl From<Random> for EphemeralSecret {
    fn from(mut rng: Random) -> EphemeralSecret {
        Self(Scalar::random(&mut rng))
    }
}

impl EphemeralSecret {
    /// Generate a `EphemeralSecret` using a new scalar mod the group
    /// order.
    pub fn new<T>(mut rng: T) -> EphemeralSecret
    where
        T: RngCore + CryptoRng,
    {
        Self(Scalar::random(&mut rng))
    }

    /// Do Diffie-Hellman key agreement between self's secret
    /// and a peer's public key, resulting in a `SharedSecret`.
    pub fn diffie_hellman(&self, peer_public: &PublicKey) -> SharedSecret {
        SharedSecret(self.0 * peer_public.0)
    }
}

#[cfg(test)]
impl Arbitrary for EphemeralSecret {
    type Parameters = ();

    fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
        array::uniform32(any::<u8>()).prop_map_into().boxed()
    }

    type Strategy = BoxedStrategy<Self>;
}
