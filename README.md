# CPC – Concise Rust Puzzles - UNIPI

**Professional, idiomatic Rust solutions for concise algorithmic challenges**

This repository contains clean, well‑tested Rust implementations of compact algorithmic puzzles aligned with the *Competitive Programming and Contests* course (University of Pisa), taught by **Prof. Rossano Venturini**. The focus is on correctness first, then expressiveness—favoring iterator‑driven solutions and minimal auxiliary state.

## Run

```bash
# 1) Clone
git clone https://github.com/Mo5444237/cpc-rust.git
cd cpc-rust

# 2) Enter the code crate
cd challenges

# 3) (optional) update toolchain
rustup update

# 4) Run tests
cargo test                              # all tests
cargo test --test <challenge_test>      # a specific challenge's tests

```

## General Structure (multiple challenges)

```
cpc-rust/
├─ README.md                     # root overview (this file)
├─ .gitignore
└─ challenges/                   # Rust library crate with all challenge code
   ├─ Cargo.lock
   ├─ Cargo.toml
   │
   ├─ src/
   │  ├─ lib.rs                  # re-export challenge modules
   │  ├─ chX/                    # one folder per challenge (X, Y, Z, ...)
   │  │  ├─ mod.rs               # implementation
   │  │  └─ README.md            # problem, approaches, chosen solution
   │  └─ ...                     # additional challenges
   │
   ├─ tests/                     # black‑box tests (one file per challenge)
   │  ├─ <challenge_test>.rs
   │  └─ ...
   │
   └─ examples/                  # optional tiny runners / demos
      └─ <optional_example>.rs
```

## Philosophy

* Small, readable functions with precise behavior and thorough tests.
* Standard library first; `itertools` used only when it improves clarity/conciseness.
* Document alternatives and complexity in each challenge’s README.
