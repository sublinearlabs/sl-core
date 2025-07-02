# SL-Core: Zero-Knowledge Proof Libraries

A comprehensive suite of Rust libraries for building sophisticated zero-knowledge protocols and applications. SL-Core provides modular, high-performance implementations of core cryptographic primitives and proof systems.

## Overview

This workspace contains several specialized crates that work together to provide a complete toolkit for zero-knowledge proof development:

- **Circuit representations** for arithmetic computations
- **Polynomial operations** including multilinear extensions
- **Interactive proof protocols** like GKR and Sumcheck
- **Field arithmetic** supporting both base and extension fields
- **Transcript management** for non-interactive proofs

## Crates

### 🔌 [`circuits`](./circuits/)
Implementation of layered arithmetic circuits with support for GKR (Goldwasser-Kalai-Rothblum) protocols.

**Features:**
- Layered circuit representation with ADD and MUL gates
- Circuit execution and evaluation traces
- GKR protocol integration with multilinear extensions
- Support for both deterministic and randomized circuit generation

### 🔢 [`fields`](./fields/)
Unified field arithmetic supporting both base fields and extension fields.

**Features:**
- Generic `Fields<F, E>` enum for seamless base/extension field operations
- Arithmetic operations that automatically handle field promotions
- Conversion utilities and type safety

### 📊 [`poly`](./poly/)
Comprehensive polynomial operations with focus on multilinear extensions.

**Features:**
- Dense multilinear polynomial representations
- Virtual polynomials (VPoly) for complex polynomial combinations
- Efficient partial evaluation and sum-over-hypercube operations
- Barycentric evaluation for univariate polynomials

### ✅ [`sum_check`](./iops/sum_check/)
Implementation of the sumcheck interactive proof protocol.

**Features:**
- Complete prover and verifier for sumcheck protocol
- Support for partial verification (useful in GKR)
- Padded sumcheck for handling non-power-of-two polynomials
- Generic over different polynomial types

### 📝 [`transcript`](./transcript/)
Fiat-Shamir transcript management for converting interactive proofs to non-interactive ones.

**Features:**
- Keccak-based challenge generation
- Support for both base and extension field elements
- Serialization-friendly design

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
# Individual crates
circuits = { git = "https://github.com/sublinearlabs/sl-core.git" }
poly = { git = "https://github.com/sublinearlabs/sl-core.git" }
sum_check = { git = "https://github.com/sublinearlabs/sl-core.git" }
fields = { git = "https://github.com/sublinearlabs/sl-core.git" }
transcript = { git = "https://github.com/sublinearlabs/sl-core.git" }
```

### Example: Creating and Evaluating a Circuit

```rust
use circuits::{LayeredCircuit, CircuitTr};
use circuits::layered_circuit::primitives::{Layer, Gate, GateOp};
use p3_goldilocks::Goldilocks as F;
use poly::Fields;

// Create a simple circuit: (a + b) * (c + d)
let layer1 = Layer::new(vec![
    Gate::new(GateOp::Add, [0, 1]), // a + b
    Gate::new(GateOp::Add, [2, 3]), // c + d
]);
let layer2 = Layer::new(vec![
    Gate::new(GateOp::Mul, [0, 1]), // (a + b) * (c + d)
]);

let circuit = LayeredCircuit::new(vec![layer1, layer2]);

// Execute with inputs [1, 2, 3, 4]
let input = [1, 2, 3, 4]
    .into_iter()
    .map(|x| Fields::Base(F::from_canonical_u32(x)))
    .collect::<Vec<_>>();

let result = circuit.execute(&input);
println!("Circuit output: {:?}", result.layers.last());
```

### Example: Sumcheck Protocol

```rust
use sum_check::{SumCheck, SumCheckInterface};
use poly::{MultilinearExtension, mle::MultilinearPoly};
use transcript::Transcript;
use p3_mersenne_31::Mersenne31 as F;
use p3_field::extension::BinomialExtensionField;

type E = BinomialExtensionField<F, 3>;

// Create a multilinear polynomial
let poly = MultilinearPoly::new_from_vec(
    3, // 3 variables
    vec![0, 0, 0, 3, 0, 0, 2, 5] // evaluations over {0,1}^3
        .into_iter()
        .map(|x| Fields::Base(F::new(x)))
        .collect()
);

let claimed_sum = poly.sum_over_hypercube();
let mut transcript = Transcript::init();

// Generate proof
let proof = SumCheck::prove(claimed_sum, poly.clone(), &mut transcript).unwrap();

// Verify proof
let mut verify_transcript = Transcript::init();
let is_valid = SumCheck::verify(&poly, &proof, &mut verify_transcript).unwrap();
assert!(is_valid);
```

## Building and Testing

```bash
# Build all crates
cargo build

# Run tests
cargo test

# Run tests with all features
cargo test --all-features

# Run benchmarks
cargo bench

# Check formatting
cargo fmt --all --check

# Run clippy
cargo clippy --workspace --all-targets --all-features
```

## Development Setup

This project uses Rust nightly. The toolchain is specified in [`rust-toolchain.toml`](./rust-toolchain.toml).

### Dependencies

The project primarily uses:
- **Plonky3** ecosystem for field arithmetic and cryptographic primitives
- **p3-field**, **p3-challenger**, **p3-mersenne-31**, **p3-goldilocks** for core functionality
- **criterion** for benchmarking
- **anyhow** for error handling

### CI/CD

The project includes GitHub Actions workflows for:
- Testing across multiple configurations
- Documentation generation
- Code formatting and linting
- Clippy analysis

## Architecture

```
sl-core/
├── circuits/          # Circuit representations and GKR, Libra, Virgo, support
├── fields/           # Unified field arithmetic
├── poly/             # Polynomial operations and MLE
├── iops/
│   └── sum_check/    # Sumcheck protocol implementation
└── transcript/       # Fiat-Shamir transcript management
```

Each crate is designed to be:
- **Modular**: Can be used independently or together
- **Generic**: Works with different field types and configurations  
- **Performant**: Optimized for cryptographic workloads
- **Safe**: Leverages Rust's type system for correctness

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes with tests
4. Ensure all CI checks pass
5. Submit a pull request

Please ensure:
- Code is formatted with `cargo fmt`
- All tests pass with `cargo test`
- Clippy warnings are addressed
- New features include documentation and tests

## License

This project is licensed under the MIT License.

## Acknowledgments

Built on the excellent [Plonky3](https://github.com/Plonky3/Plonky3) framework for zero-knowledge proof systems.