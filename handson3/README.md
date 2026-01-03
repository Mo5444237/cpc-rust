# HandsOn 3 — Dynamic Programming

**Source:** [Professor Rossano Venturini's HandsOn 3](https://pages.di.unipi.it/rossano/blog/2024/handson3/)

HandsOn 3 focuses on Dynamic Programming techniques in Rust. It consists of two independent problems, both solved using efficient DP strategies with different optimization approaches.

## Problem 1 — Holiday Planning

**Goal:** Plan a Christmas vacation across multiple cities to maximize attractions visited.

**Given:**
* `n` cities, each with a daily itinerary of attractions
* `D` total vacation days
* Must visit attractions in order (no cherry-picking)

**Challenge:** Decide how many days to spend in each city to maximize total attractions.

**Solution:** Use **Dynamic Programming** with:
* **State:** `dp[i][j]` = max attractions using first `i` cities with `j` days
* **Precomputation:** Cumulative attractions for each city
* **Transition:** Try all possible day allocations for each city

This yields **O(n × D²)** time complexity.

**Detailed explanation:** [problem1.md](./problem1.md)

## Problem 2 — Design a Course

**Goal:** Select maximum number of course topics satisfying dual constraints.

**Given:**
* `n` topics, each with beauty and difficulty values
* Must have strictly increasing beauty
* Must have strictly increasing difficulty

**Challenge:** Both dimensions must increase simultaneously.

**Solution:** Use **Sort + Longest Increasing Subsequence (LIS)**:
* **Sort** by beauty ascending (difficulty descending for ties)
* **Find LIS** of difficulty values using binary search
* Sorting reduces 2D problem to 1D LIS

This yields **O(n log n)** time complexity.

**Detailed explanation:** [problem2.md](./problem2.md)

## Testing

Each problem includes a dedicated test suite using file-based test cases:

* `tests/problem1.rs` → validates Holiday Planning DP implementation
* `tests/problem2.rs` → validates Course Design LIS implementation

Place all input/output files under:

```
handson3/tests/data/problem1/
handson3/tests/data/problem2/
```

Run all tests:

```bash
cargo test
```

## Project Structure (HandsOn 3)

```
handson3/
├── README.md            # Overview (this file)
├── problem1.md          # Full Problem 1 Documentation
├── problem2.md          # Full Problem 2 Documentation
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

* **Problem 1:** Resource allocation DP with O(n × D²) complexity for vacation planning.
* **Problem 2:** Sort + LIS with O(n log n) complexity for 2D constraint optimization.

For implementation details and algorithmic explanations, refer to each dedicated problem file.
