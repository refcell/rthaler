<img align="right" width="150" height="150" top="100" src="./assets/readme.jpg">

# rthaler • [![tests](https://github.com/abigger87/rthaler/actions/workflows/tests.yaml/badge.svg)](https://github.com/abigger87/rthaler/actions/workflows/tests.yaml) [![lints](https://github.com/abigger87/rthaler/actions/workflows/lints.yaml/badge.svg)](https://github.com/abigger87/rthaler/actions/workflows/lints.yaml) ![GitHub](https://img.shields.io/github/license/abigger87/rthaler)  ![Crates.io](https://img.shields.io/crates/v/rthaler)

Dr. Thaler's book [Proofs, Args, and ZK](https://people.cs.georgetown.edu/jthaler/ProofsArgsAndZK.pdf) implemented in rust using the [arkworks](https://arkworks.rs) cryptographic rust toolset.

Various Zero Knowledge Protocols are implemented at the subdirectory level. For interactivity between provers and verifiers, `rthaler` uses the [actix](https://actix.rs/) [actor framework](https://docs.rs/actix/0.13.0/actix/) to pass messages between the provers and verifiers. In order to do this, both the prover and the verifier implement actix's [Actor Trait](https://docs.rs/actix/0.13.0/actix/trait.Actor.html).



## Implementations

**Sum Check**

[`./sumcheck`](./sumcheck/) implements the sum-check protocol following [0xsage](https://medium.com/@0xsage)'s Medium [tutotial](https://medium.com/yearofzk/rust-guide-sum-check-protocol-18ceb8affdb2).



## Usage

**Build**
```bash
cargo build
```

**Run Tests**
```bash
cargo test
```


## Blueprint

```ml
utils
├─ refactored utilities
sumcheck
├─ 
```


## Contributing

All contributions are welcome! We want to make contributing to this project as easy and transparent as possible, whether it's:
  - Reporting a bug
  - Discussing the current state of the code
  - Submitting a fix
  - Proposing new features
  - Becoming a maintainer

Please be aware, when you submit code changes, your submissions are understood to be under the same [THE UNLICENSE](https://github.com/abigger87/rthaler/blob/master/LICENSE) that covers the project. Feel free to contact the maintainers if that's a concern.

We use GitHub issues to track public bugs. Report a bug by [opening a new issue](https://github.com/abigger87/rthaler/issues/new); it's that easy!


## License

All code and contributions made directly to this repository are expressly understood to be licensed under the [UNLICENSE](https://github.com/abigger87/rthaler/blob/master/LICENSE) as aforementioned.

## References

- [0xsage](https://medium.com/@0xsage)'s walkthrough of implementing the sum-check protocol with arkworks on [medium](https://medium.com/yearofzk/rust-guide-sum-check-protocol-18ceb8affdb2)
- [Contributing Example](https://gist.github.com/briandk/3d2e8b3ec8daf5a27a62)
- [Production Grade Logging in Rust](https://betterprogramming.pub/production-grade-logging-in-rust-applications-2c7fffd108a6)
