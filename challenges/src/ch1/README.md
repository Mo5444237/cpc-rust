# Challenge #1 — Maximum Parentheses Depth

**Task.** Implement

```rust
pub fn max_depth(s: &str) -> usize
```

Return the maximum nesting depth of parentheses in `s`. Non‑parenthesis characters are ignored. If stray `)` appear, depth never goes below 0.

**Examples**

* `( a(b) (c) (d(e(f)g)h) I (j(k)l)m)` → `4`
* `((()))` → `3`
* `(a(b(c)d)e)` → `3`
* `abc` → `0`


## Possible Solutions & Complexities

### 1) Counter (stack‑free) — **Chosen**

* **Idea:** Track two integers while scanning:

  * `curr`: current number of unmatched `'('`.
  * `max`: maximum value of `curr` seen so far.
* **Rules:**

  * On `'('`: `curr += 1; max = max.max(curr)`
  * On `')'`: `curr = curr.saturating_sub(1)`
  * Otherwise: ignore
* **Time:** `O(n)`
* **Space:** `O(1)`
* **Why good:** Single pass, constant space, easy to prove correct; robust against stray `)` via `saturating_sub`.

### 2) Explicit Stack

* Push on `'('`, pop on `')'`, track max stack height.
* **Time:** `O(n)`
* **Space:** `O(n)` (unnecessary overhead vs. counter)

### 3) Regex/Parser Frameworks

* Overkill and slower; adds dependencies/complexity with no benefit here.


## Why This Solution

The counter method captures the exact invariant we need (number of currently open parentheses) with minimal state and maximum clarity. It’s also the most cache‑friendly and avoids allocations.

## Rust Implementation (iterator fold)

```rust
pub fn max_depth(s: &str) -> usize {
    s.chars()
        .fold((0usize, 0usize), |(curr, max), ch| match ch {
            '(' => {
                let curr = curr + 1;
                (curr, max.max(curr))
            }
            ')' => (curr.saturating_sub(1), max),
            _ => (curr, max),
        })
        .1
}
```

### How it works (step by step)

* `s.chars()` iterates Unicode scalar values; we only react to `'('` and `')'`.
* `fold((0, 0), ...)` carries a tuple `(curr, max)` across the scan.
* On `'('`, we increment `curr` and possibly update `max`.
* On `')'`, we decrement with `saturating_sub(1)` to avoid underflow if the input has stray closings.
* The fold returns the final `(curr, max)`; we extract `.1` (the `max`).

> Alternative imperative version is equivalent; we prefer the fold when it stays readable.


## Correctness (sketch)

Let `curr(i)` be the number of unmatched `'('` after processing the first `i` characters. The depth of the string equals `max_i curr(i)`. The algorithm maintains exactly this `curr` and records its maximum, hence it returns the correct depth.


## Tests (subset)

```rust
assert_eq!(max_depth("((()))"), 3);
assert_eq!(max_depth("(a(b(c)d)e)"), 3);
assert_eq!(max_depth("abc"), 0);
assert_eq!(max_depth("( a(b) (c) (d(e(f)g)h) I (j(k)l)m)"), 4);
assert_eq!(max_depth(""), 0);
```
