use actix::{Message, MessageResponse};
use serde::{Deserialize, Serialize};
use crate::types::*;

/// ## ProofMessage
///
/// A message to be exchanged between prover and verifier actors.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ProofMessage<S> {
    /// Proof
    /// verifier ---> prover
    Proof(S),
    /// Verification Request
    /// user ---> verifier
    Verify(S),
    /// Verification Request (mocked slowly)
    /// user ---> verifier
    SlowVerify(S),
    /// Successful Verification
    Success,
}

impl<S> Message for ProofMessage<S> {
    type Result = ProofResponse;
}

/// ## ProofResponse
///
/// A response to a proof message.
#[derive(Serialize, Deserialize, MessageResponse)]
pub enum ProofResponse {
    /// A polynomial Response
    #[serde(skip)]
    MultiPoly(MultiPoly),
    /// A scalar Response
    #[serde(skip)]
    UniPoly(UniPoly),
    /// Proof Failed
    Failed,
    /// Unimplemented endpoint
    Unimplemented,
}
