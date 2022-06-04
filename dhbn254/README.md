## Diffie-Hellman-Merkle Key Exchange Protocol

Implements the [Diffie-Hellman-Merkle Key Exchange Protocol](https://en.wikipedia.org/wiki/Diffie%E2%80%93Hellman_key_exchange) using [arkworks](https://arkworks.rs) bn254 elliptic curve.

> [arkworks](https://arkworks.rs) [bn254](https://docs.rs/ark-bn254/0.3.0/ark_bn254/) implements the curve sampled from [BCTV14 paper](https://eprint.iacr.org/2013/879.pdf). The name denotes that it is a Barreto–Naehrig curve of embedding degree 12, defined over a 254-bit (prime) field. The scalar field is highly 2-adic.


#### WARNING

```md
This curve does not satisfy the 128-bit security level anymore.
```

#### Example

**Memory DH Key Exchange**
```rust
use rand_core::OsRng;

use dhbn254::EphemeralSecret;
use dhbn254::PublicKey;

// Alice's side
let alice_secret = EphemeralSecret::new(&mut OsRng);
let alice_public = PublicKey::from(&alice_secret);

// Bob's side
let bob_secret = EphemeralSecret::new(&mut OsRng);
let bob_public = PublicKey::from(&bob_secret);

// Alice again
let alice_shared_secret = alice_secret.diffie_hellman(&bob_public);

// Bob again
let bob_shared_secret = bob_secret.diffie_hellman(&alice_public);

// Each peer's computed shared secret should be the same.
assert_eq!(<[u8; 32]>::from(alice_shared_secret), <[u8; 32]>::from(bob_shared_secret));
```

**Message-Passed Key Exchange**
```rust
use rand_core::OsRng;

use dhbn254::EphemeralSecret;
use dhbn254::PublicKey;

// Alice's side
let alice_secret = EphemeralSecret::new(&mut OsRng);
let alice_public = PublicKey::from(&alice_secret);

// Bob's side
let bob_secret = EphemeralSecret::new(&mut OsRng);
let bob_public = PublicKey::from(&bob_secret);

// Alice again
let alice_shared_secret = alice_secret.diffie_hellman(&bob_public);

// Bob again
let bob_shared_secret = bob_secret.diffie_hellman(&alice_public);

// Each peer's computed shared secret should be the same.
assert_eq!(<[u8; 32]>::from(alice_shared_secret), <[u8; 32]>::from(bob_shared_secret));
```