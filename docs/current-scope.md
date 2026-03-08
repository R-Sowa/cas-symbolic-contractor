# Current Scope

## Implemented Today

The current prototype includes:

- typed `Core / Active / Virtual / General` indices
- fermionic creation / annihilation operator strings
- deterministic fermionic normal ordering
- CAS-aware symbolic reduction for a supported low-order slice
- reduction into:
  - `delta`
  - `gamma` (1-RDM)
  - `Gamma` (2-RDM)
  - `HigherRDM` for unresolved higher-order active-space structure
- expectation values and short matrix elements

## Not Implemented

The current prototype does not provide:

- a full generalized Wick theorem implementation
- a complete higher-order multireference contraction engine
- numerical CASCI / CASSCF
- CASPT2 or related perturbative solvers
- parser-first or DSL-first interfaces

## Reliability Note

This repository includes internal tests and exact small-system cross-checks for the currently supported slice.

However, it remains an early-stage hackathon prototype built with substantial AI assistance.
It should not be treated as a validated scientific software package, and its scientific reliability should not be assumed beyond the explicitly tested scope.

## Intended Use

This repository is best read as:

- a prototype algebra core
- a reference implementation for low-order CAS-aware symbolic contraction
- a starting point for future refinement, validation, and extension
