# HandsOn 2 — Problem 2: Is There?

## Problem Overview

You are given **n segments** on a number line. Each segment is:

```
[L_i, R_i]   (with L_i ≤ R_i)
```

Then you receive **m queries** of the form:

```
IsThere(i, j, k)
```

The query asks:

> **Does there exist a position x such that i ≤ x ≤ j and exactly k segments cover x?**

The answer is:

* **1** if such an x exists
* **0** otherwise

The solution must run in **O((n + m) log n)**.

## Key Challenge

If we try to check each position inside each query range, the worst case becomes:

* up to **n** segments
* up to **10⁵ queries**

which is far too slow.

We need a way to:

1. Compute how many segments cover each position **efficiently**
2. Answer queries about whether a value **k** exists inside a range **[i, j]**
3. Do everything without scanning the whole range every time

## Step 1 — Build Coverage Array Using Difference Array + Prefix Sum

We want to compute an array `coverage[x]` = number of segments covering point x.

A classic trick:

Let `diff[]` be initially all zeros.
For each segment `[L, R]`:

```
diff[L] += 1
diff[R + 1] -= 1
```

After processing all segments, take prefix sums:

```
coverage[x] = diff[0] + diff[1] + ... + diff[x]
```

This produces the correct number of covering segments for every position.

### Example

Segments:

```
[0, 4]
[1, 3]
[1, 2]
[1, 1]
[0, 0]
```

Difference array after all updates:

```
idx: 0  1  2  3  4  5
val: 2  3  1 -1 -1 -0
```

Prefix sum → coverage:

```
idx: 0 1 2 3 4
cov: 2 5 6 5 4
```

So:

* at x=2 → covered by 6 segments
* at x=4 → covered by 4 segments

This gives us everything we need for the next step.

## Step 2 — Build a Segment Tree that Stores (min, max)

Each node stores:

* `min_value` in its range
* `max_value` in its range

Why?

A query asks:

> is there an x ∈ [i, j] such that coverage[x] = k?

Let the segment tree node covering this interval have:

```
(min_value, max_value)
```

Then:

* if `k < min_value` → impossible
* if `k > max_value` → impossible
* otherwise, **maybe** — must search deeper

This is enough to prune entire subtrees immediately.

## Step 3 — Answer Queries with Pruning

To answer `IsThere(i, j, k)`:

We recursively search the segment tree:

### Rules

1. **No overlap** → return false
2. **Range fully inside the query**:

   * if `k < min_value or k > max_value` → return false
   * if `min_value == max_value == k` → return true (entire segment is exactly k!)
3. **Partial overlap** → recurse left and right

We stop as soon as we find a leaf with value `k`.

Because min/max prune large parts of the tree, this search is **O(log n)**.


## Full Rust Implementation

(See `src/problem2.rs` in this repository for the final code.)

The implementation includes:

* Efficient coverage computation
* A segment tree storing `min_value` and `max_value` per node
* A recursive existence check with pruning


## Correctness Sketch

### 1. Coverage computation

The difference-array + prefix-sum correctly computes how many segments cover each point because each segment contributes:

* `+1` from its left endpoint
* `-1` after its right endpoint

This produces exact coverage after prefix summation.

### 2. Segment tree correctness

Each node maintains:

* `min_value = min coverage in range`
* `max_value = max coverage in range`

Thus, for query `k`:

* If `k` outside this range → impossible
* If `min_value == max_value == k` → guaranteed true
* Otherwise, check children

### 3. Time complexity

* Building coverage: **O(n)**
* Building segment tree: **O(n)**
* Each query: **O(log n)**

Total: **O((n + m) log n)**.

## Example Query Walkthrough

Given coverage = `[2, 5, 6, 5, 4]`

Query: `IsThere(1, 3, 5)` → do we have five coverings in `[1, 3]`?

* Node covering `[1, 3]` has `(min=5, max=6)` → k=5 is inside
* Recurse
* At `[1, 1]`: value = 5 → match → return true


## Summary

Problem 2 is solved using:

1. **Difference array → prefix sum** to compute coverage
2. **Segment tree with min/max** to prune impossible ranges
3. **Target search** to check whether value `k` exists in a query range

This combination yields an efficient and clean solution within the required complexity.
