# CPC – Concise Rust Puzzles - UNIPI

**Professional, idiomatic Rust solutions for concise algorithmic challenges**

This repository contains clean, well‑tested Rust implementations of compact algorithmic puzzles aligned with the *Competitive Programming and Contests* course (University of Pisa), taught by **Prof. Rossano Venturini**. The focus is on correctness first, then expressiveness—favoring iterator‑driven solutions and minimal auxiliary state.

## Repository Structure

This repository is organized into two main sections:

### 1. **Challenges** — Weekly algorithmic puzzles
Small, focused problems solved with idiomatic Rust patterns.

### 2. **HandsOn** — Comprehensive implementations
Larger exercises involving data structures and algorithms (binary trees, graphs, etc.).


## Run

### Challenges
```bash
# 1) Clone
git clone https://github.com/Mo5444237/cpc-rust.git
cd cpc-rust

# 2) Enter the challenges crate
cd challenges

# 3) (optional) update toolchain
rustup update

# 4) Run tests
cargo test                              # all tests
cargo test --test <challenge_test>      # a specific challenge's tests
```

### HandsOn
```bash
# 1) From repository root
cd handson1                             # or handson2, handson3, etc.

# 2) Run tests
cargo test                              # all tests for this HandsOn
```


## General Structure

```
cpc-rust/
├─ README.md                     # root overview (this file)
├─ Cargo.toml                    # workspace configuration
├─ .gitignore
│
├─ challenges/                   # Weekly algorithmic challenges
│  ├─ Cargo.toml
│  ├─ src/
│  │  ├─ lib.rs                  # re-export challenge modules
│  │  ├─ chX/                    # one folder per challenge
│  │  │  ├─ mod.rs               # implementation
│  │  │  └─ README.md            # problem, approaches, chosen solution
│  │  └─ ...
│  │
│  └─ tests/                     # black‑box tests (one file per challenge)
│     ├─ chX_<name>.rs
│     └─ ...
│
└─ handsonX/                     # HandsOn exercises (binary trees, graphs, etc.)
   ├─ Cargo.toml
   ├─ README.md                  # detailed problem description & solutions
   └─ src/
      └─ lib.rs                  # implementation + tests
```

## Philosophy

* Small, readable functions with precise behavior and thorough tests.
* Standard library first; `itertools` used only when it improves clarity/conciseness.
* Document alternatives and complexity in each challenge’s README.
