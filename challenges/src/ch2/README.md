# Challenge #2 — Missing Number

**Task.** Given a slice containing a permutation of integers in `0..=n` with **exactly one** value missing, return the missing value.

**Examples**

* `[3, 0, 1]` → `2`
* `[0, 1, 2, 4]` → `3`
* `[5, 2, 0, 1, 3]` → `4`
* `[1]` → `0`

## Possible Solutions & Complexities

### 1) XOR trick — **Chosen**

* **Idea:** XOR all numbers in `0..=n` and XOR all elements of `a`. Equal values cancel (`x ^ x = 0`), leaving the missing one.
* **Time:** `O(n)`
* **Space:** `O(1)`
* **Pros:** No overflow risk, very concise, single pass over `a` (+ implicit pass over `0..=n`).

### 2) Sum formula

* **Idea:** `missing = n*(n+1)/2 - sum(a)`
* **Time:** `O(n)`
* **Space:** `O(1)`
* **Note:** Use a wider integer (e.g., `u64`) to avoid overflow if types/sizes change (overflow risk).

### 3) Sorting

* Sort and find first index `i` where `a[i] != i`.
* **Time:** `O(n log n)`
* **Space:** `O(1)` or `O(n)` depending on the sort; slower than needed here.

### 4) Hash/bitset presence map

* Mark seen values, then scan `0..=n`.
* **Time:** `O(n)`
* **Space:** `O(n)` (unnecessary extra memory here).


## Why This Solution

The XOR method is optimal and safe: no arithmetic overflow, constant extra space, and simple to reason about (pairwise cancellation). It’s also very expressive with iterator folds in Rust.


## Rust Implementation

```rust
pub fn missing_number(a: &[u32]) -> u32 {
    let n = a.len() as u32;                 // since one value is missing from 0..=n
    let all = (0..=n).fold(0, |x, i| x ^ i);
    let arr = a.iter().copied().fold(0, |x, i| x ^ i);
    all ^ arr
}
```

### How it works (step by step)

* Compute `all = 0 ^ 1 ^ 2 ^ … ^ n`.
* Compute `arr = a[0] ^ a[1] ^ … ^ a[n-1]`.
* Every number that appears in both sets cancels to `0`; only the missing number remains when we XOR `all ^ arr`.


## Correctness & Complexity

* **Correctness:** The two multisets `{0..=n}` and `{a}` differ by exactly one element; XOR of their elements yields that element.
* **Time:** `O(n)`
* **Space:** `O(1)`


## Tests (subset)

```rust
assert_eq!(missing_number(&[3, 0, 1]), 2);
assert_eq!(missing_number(&[0, 1, 2, 4]), 3);
assert_eq!(missing_number(&[5, 2, 0, 1, 3]), 4);
assert_eq!(missing_number(&[1]), 0);
assert_eq!(missing_number(&[]), 0);
```
