# CAS Symbolic Contractor

A Rust prototype for fermionic normal ordering and CAS-aware symbolic contraction on CASCI/CASSCF references.

## Status / Disclaimer

This repository is an early-stage hackathon prototype built with substantial AI assistance.
It is still under active development, and neither the code quality nor the scientific reliability should be considered validated yet.

## What This Repository Does

Current prototype scope:

- fermionic creation / annihilation operator algebra
- typed `Core / Active / Virtual / General` index spaces
- fermionic normal ordering
- low-order CAS-aware symbolic reduction
- reduction into `delta`, `gamma` (1-RDM), `Gamma` (2-RDM), and unresolved `HigherRDM` structure
- expectation values and short matrix elements

## What This Repository Does Not Do

This project is not:

- a numerical CASSCF implementation
- a CASPT2 solver
- a complete generalized Wick theorem engine
- a validated scientific software package

## Quick Links

- `docs/quick-start.md`
- `docs/current-scope.md`

## Local Development

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
cargo doc --no-deps
```

## Demo

```bash
cargo run --bin demo
cargo run --bin demo -- --trace
cargo run --bin demo -- --cross-check
```
