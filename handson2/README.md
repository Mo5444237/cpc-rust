# HandsOn 2 — Segment Trees & Coverage Queries

**Source:** [Professor Rossano Venturini's HandsOn 2](https://pages.di.unipi.it/rossano/blog/2024/handson2/)

HandsOn 2 focuses on advanced range-query data structures. It consists of two independent problems, both solved using efficient tree‑based techniques.


## Problem 1 — Min and Max (Range ChMin + Range Max)

**Goal:** Maintain an array under two types of operations:

* `Update(i, j, T)` → apply `A[k] = min(A[k], T)` for all `k ∈ [i, j]`
* `Max(i, j)` → return the maximum value in `A[i..j]`

**Challenge:** The update is a *range chmin* — simple lazy propagation does not work.

**Solution:** Implement **Segment Tree Beats**, storing:

* `max_value` = largest value in a range
* `second_max` = second-largest value

This allows efficient conditional propagation and supports both operations in **O(log n)**.

**Detailed explanation:** [problem1.md](./problem1.md)


## Problem 2 — Is There?

Given `n` segments and queries of the form `IsThere(i, j, k)`, determine whether:

> There exists an `x` with `i ≤ x ≤ j` such that exactly `k` segments cover position `x`.

**Steps to Solve:**

1. Compute a coverage array using **difference array + prefix sums**.
2. Build a **min/max segment tree** over the coverage.
3. For each query, check if `k` exists in `[i, j]` using:

   * pruning by `(min, max)` bounds
   * recursive search for a matching leaf

This yields **O((n + m) log n)** total complexity.

**Detailed explanation:** [problem2.md](./problem2.md)


## Testing

Each problem includes a dedicated test suite using file-based test cases:

* `tests/problem1.rs` → validates Segment Tree Beats implementation
* `tests/problem2.rs` → validates coverage + existence queries

Place all input/output files under:

```
handson2/tests/data/problem1/
handson2/tests/data/problem2/
```

Run all tests:

```
cargo test
```

## Project Structure (HandsOn 2)

```
handson2/
├── README.md            # Overview (this file)
├── problem1.md          # Full Problem 1 write-up
├── problem2.md          # Full Problem 2 write-up
├── src/
│   ├── lib.rs
│   ├── problem1.rs
│   └── problem2.rs
└── tests/
    ├── problem1.rs
    ├── problem2.rs
    └── data/
        ├── problem1/
        └── problem2/
```

## Summary

* **Problem 1:** Segment Tree Beats for range chmin + max.
* **Problem 2:** Difference array + prefix coverage + segment tree value existence.

For implementation details and algorithmic explanations, refer to each dedicated problem file.
