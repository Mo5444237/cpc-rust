# Challenge #3 — Majority Element (> ⌊n/2⌋)

**Task.** Implement

```rust
pub fn majority(a: &[u32]) -> Option<u32>
```

Return the element that appears **more than** ⌊n/2⌋ times in `a` (if it exists), otherwise `None`.

**Examples**

* `[3, 3, 4, 2, 3, 3, 5]` → `Some(3)`
* `[1, 2, 3, 4]` → `None`
* `[2, 2, 1, 1, 2, 2, 2]` → `Some(2)`


## Possible Solutions & Complexities

### 1) Boyer–Moore Majority Vote — **Chosen**

* **Idea:** Maintain a `candidate` and a `count`. When the next number equals `candidate`, increment; otherwise decrement. If `count` hits 0, set the next number as the new `candidate` with `count = 1`. This cancels pairs of different values.
* **Verification:** A second pass confirms the candidate occurs `> ⌊n/2⌋`.
* **Time:** `O(n)`
* **Space:** `O(1)`

### 2) HashMap counting

* Count frequencies and check if any exceeds `⌊n/2⌋`.
* **Time:** `O(n)`
* **Space:** `O(n)`

### 3) Sorting

* Sort, take the middle element as a candidate, then verify.
* **Time:** `O(n log n)`
* **Space:** implementation‑dependent


## Why This Solution

Boyer–Moore achieves optimal time and space with a simple cancellation invariant. The extra verification pass ensures correctness when no majority exists.


## Rust Implementation

```rust
pub fn majority(a: &[u32]) -> Option<u32> {
    // Phase 1: Find candidate using Boyer-Moore voting
    let candidate = a
        .iter()
        .fold((0, 0), |(cand, count), &x| {
            if count == 0 {
                (x, 1)
            } else if x == cand {
                (cand, count + 1)
            } else {
                (cand, count - 1)
            }
        })
        .0;

    // Phase 2: Verify candidate is majority (appears > n/2 times)
    (a.iter().filter(|&&x| x == candidate).count() > a.len() / 2).then_some(candidate)
}
```

### How it works

* **Phase 1 (candidate):** pairs of different values "cancel"; a true majority cannot be fully canceled and remains as the candidate.
* **Phase 2 (verify):** count occurrences of the candidate and check if it exceeds `n/2`.

### Code Breakdown

* `a.iter()` yields **`&u32` references**. In the fold closure we pattern‑match `&x` to copy the `u32` by value (scalars implement `Copy`).
* `fold((0, 0), |(cand, count), &x| { ... })`:

  * **Seed:** `(0, 0)` means `candidate_value = 0`, `count = 0` initially.
  * **Accumulator:** the closure receives the previous state `(cand, count)` and the next element `x`, and returns the **new** state tuple.
  * **Why a tuple?** It’s a compact way to carry two pieces of state without a custom struct. The compiler turns this into an efficient loop.
  * **Equivalent loop:**

    ```rust
    let mut cand = 0u32; let mut count = 0i32;
    for &x in a { /* same if/else logic */ }
    ```
* The if/else chain implements Boyer–Moore:

  * `count == 0` → start a new candidate `(x, 1)`
  * `x == cand` → `count + 1`
  * otherwise → `count - 1`
    We use `i32` for `count` because it naturally supports `+1/-1`.
* Verification uses an **iterator pipeline**:

  * `a.iter()` → references to items
  * `.filter(|&&y| y == candidate)` → keep only elements equal to the candidate

    * Note the `&&y`: `iter()` yields `&u32`; the `filter` closure receives `&&u32` (a reference to the iterator’s item), so we destructure to get the `u32` value.
    * An alternative is `.iter().copied().filter(|&y| y == candidate)`.
  * `.count()` consumes the iterator and returns how many items matched.
* `(predicate).then_some(candidate)` is a neat way to write:

  ```rust
  if predicate { Some(candidate) } else { None }
  ```

  It keeps the code expression‑oriented and avoids a separate `if` block.


## Correctness (sketch)

If a majority exists, it outnumbers all other elements combined, so it cannot be eliminated by the cancellations in Phase 1 and must be the final candidate. If no majority exists, Phase 2 returns `None`.


## Tests (subset)

```rust
assert_eq!(majority(&[3, 3, 4, 2, 3, 3, 5]), Some(3));
assert_eq!(majority(&[1, 2, 3, 4]), None);
assert_eq!(majority(&[2, 2, 1, 1, 2, 2, 2]), Some(2));

// Edge cases
assert_eq!(majority(&[]), None);
assert_eq!(majority(&[7]), Some(7));
assert_eq!(majority(&[1, 2, 2, 1]), None);
```
