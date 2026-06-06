# silo-core

**Core math silo for the SuperInstance ternary fleet.**

Separates pure Z₃ computation (the **Silo**) from the Pincher reflex runtime.
All ternary math crates in the fleet depend on `silo-core` → `ternary-types`,
forming a clean dependency hierarchy.

## Architecture

```text
┌──────────────────────────────────────────────────┐
│              Pincher (reflex runtime)             │
│    Vector DB, reflexes, agents, persistence      │
├──────────────────────────────────────────────────┤
│            ternary-math-crates (24+)              │
│  (matmul, conv, fuse, hmm, knn, prune, …)       │
├─────────────────── SILO ─────────────────────────┤
│                  silo-core                        │
│   Core algebra, geometry, analysis on Z₃         │
├──────────────────────────────────────────────────┤
│                ternary-types                      │
│    Ternary enum, TritVector, Matrix, PackedTrits │
└──────────────────────────────────────────────────┘
```

| Layer | Role | Dependencies |
|-------|------|--------------|
| **Silo** (`silo-core`) | Pure math on Z₃ | `ternary-types` only |
| **Pincher** | Runtime, reflexes, persistence | `silo-core` + `ternary-types` |

## The Silo Pattern

The Silo/Pincher separation solves the "Ternary Connection Gap":

1. **Silo** owns the math — algebra, geometry, analysis, and invariants.
   - Has zero knowledge of Pincher
   - Ships pure computations that can run anywhere
   - Is the canonical source of truth for Z₃ operations

2. **Pincher** owns the runtime — vector database, reflex engine, persistence.
   - Depends on the Silo for math
   - Adds state, persistence, and real-time reactivity

3. **Ternary crates** (the 24+ math-stack crates) operate in the Silo layer.
   - Each depends on `ternary-types` for the shared type schema
   - Each can optionally depend on `silo-core` for higher-level math

## Modules

| Module | Description |
|--------|-------------|
| `algebra` | Polynomial evaluation, matrix invariants, convolution over Z₃ |
| `geometry` | Distances (L1, L2, L∞), rotations, reflections, ternary spheres |
| `analysis` | Discrete calculus, autocorrelation, entropy, dynamics |

## Usage

```rust
use silo_core::Ternary::{Positive, Negative, Neutral};
use silo_core::algebra::dot;
use silo_core::geometry::manhattan_distance;
use silo_core::analysis::ternary_entropy;

let a = [Positive, Negative, Neutral];
let b = [Positive, Positive, Neutral];

let d = manhattan_distance(&a, &b);
println!("Manhattan distance: {d}");

let h = ternary_entropy(&[Positive, Negative, Neutral, Positive]);
println!("Entropy: {h:.3} bits");
```

## Connectivity

`silo-core` is the structural hub that connects the fleet:

- `ternary-types` → shared type schema
- `silo-core` → shared math operations
- Fleet crates → consume both

Target: **>80%** of ternary crates connected to `silo-core` / `ternary-types`.

## License

MIT OR Apache-2.0
