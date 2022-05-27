<img align="right" width="150" height="150" top="100" src="./assets/readme.jpg">

# rthaler • [![tests](https://github.com/abigger87/rthaler/actions/workflows/tests.yaml/badge.svg)](https://github.com/abigger87/rthaler/actions/workflows/tests.yaml) [![lints](https://github.com/abigger87/rthaler/actions/workflows/lints.yaml/badge.svg)](https://github.com/abigger87/rthaler/actions/workflows/lints.yaml) ![GitHub](https://img.shields.io/github/license/abigger87/rthaler)  ![Crates.io](https://img.shields.io/crates/v/rthaler)

Dr. Thaler's book [Proofs, Args, and ZK](https://people.cs.georgetown.edu/jthaler/ProofsArgsAndZK.pdf) implemented in rust using the [arkworks](https://arkworks.rs) cryptographic rust toolset.

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
assets
├─ embedded documentation images
sumcheck
├─ 
├─ Cargo.toml — Library Cargo Manifest
Cargo.toml — Workspace Cargo Manifest
```


## Contributing



## License

[THE UNLICENSE](https://github.com/abigger87/stub.rs/blob/master/LICENSE)
