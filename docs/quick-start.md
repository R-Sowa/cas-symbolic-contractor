# Quick Start

## Prerequisites

- Rust toolchain
- Cargo

## Build and Test

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
cargo doc --no-deps
```

## Run the Prototype Demo

Basic curated demo:

```bash
cargo run --bin demo
```

With trace stages:

```bash
cargo run --bin demo -- --trace
```

With exact cross-check output:

```bash
cargo run --bin demo -- --cross-check
```

## What to Look For

The current prototype reduces supported expressions into symbolic objects such as:

- `delta`
- `gamma` (1-RDM)
- `Gamma` (2-RDM)
- `HigherRDM` for unresolved higher-order structure

The `--cross-check` mode compares the symbolic reduction against a small-system exact evaluator for curated examples.
