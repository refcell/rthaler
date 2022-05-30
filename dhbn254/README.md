## Diffie-Hellman-Merkle Key Exchange Protocol

Implements the [Diffie-Hellman-Merkle Key Exchange Protocol](https://en.wikipedia.org/wiki/Diffie%E2%80%93Hellman_key_exchange) using [arkworks](https://arkworks.rs) bn254 elliptic curve.

> [arkworks](https://arkworks.rs) [bn254](https://docs.rs/ark-bn254/0.3.0/ark_bn254/) implements the curve sampled from [BCTV14 paper](https://eprint.iacr.org/2013/879.pdf). The name denotes that it is a Barreto–Naehrig curve of embedding degree 12, defined over a 254-bit (prime) field. The scalar field is highly 2-adic.


#### WARNING

```md
This curve does not satisfy the 128-bit security level anymore.
```

