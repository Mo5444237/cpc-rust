# Challenge #4 — Run-Length Encoding (RLE)

**Task.** Implement

```rust
pub fn rle(s: &str) -> Vec<(char, usize)>
```

Return a vector of `(character, count)` pairs representing consecutive runs of the same character in the input string `s`.

**Examples**

* `"aaabbc"` → `[('a', 3), ('b', 2), ('c', 1)]`
* `"abc"` → `[('a', 1), ('b', 1), ('c', 1)]`
* `"aaaaa"` → `[('a', 5)]`
* `"aaaabbbaaa"` → `[('a', 4), ('b', 3), ('a', 3)]`
* `""` → `[]`


## Possible Solutions & Complexities

### 1) Iterative manual scan

* Track the current character and its count.
* When the character changes, push the pair and reset the counter.
* **Time:** `O(n)`
* **Space:** `O(k)` (output size, k = number of runs)

### 2) Functional approach with `itertools::chunk_by` — **Chosen**

* Use the `chunk_by` adapter to group consecutive equal items, then map each chunk to `(char, count)`.
* **Time:** `O(n)`
* **Space:** `O(k)`


## Why This Solution

The `itertools::chunk_by` version expresses the intent clearly—"group consecutive equal items and count them"—in just a few lines. It avoids manual state tracking while staying linear‑time and memory‑safe.


## Rust Implementation

```rust
use itertools::Itertools;

pub fn rle(s: &str) -> Vec<(char, usize)> {
    s.chars()
        .chunk_by(|&c| c)
        .into_iter()
        .map(|(ch, group)| (ch, group.count()))
        .collect()
}
```

### Idiomatic Rust breakdown

* `s.chars()` yields an iterator over Unicode scalar values (`char`).
* `.chunk_by(|&c| c)` groups **consecutive** elements by equality of their key—in this case, each unique character.
* `.into_iter()` converts the grouped view into an iterator of `(key, group)` pairs.
* `.map(|(ch, group)| (ch, group.count()))` transforms each chunk into `(character, count)`.
* `.collect()` gathers the resulting pairs into a `Vec<(char, usize)>`.


## Correctness

Each contiguous sequence of identical characters is converted to exactly one `(char, count)` pair. The algorithm performs a single pass over the input, guaranteeing O(n) time.


## Tests (subset)

```rust
assert_eq!(rle("aaabbc"), vec![("a", 3), ("b", 2), ("c", 1)]);
assert_eq!(rle("abc"), vec![("a", 1), ("b", 1), ("c", 1)]);
assert_eq!(rle("aaaaa"), vec![("a", 5)]);
assert_eq!(rle("aaaabbbaaa"), vec![("a", 4), ("b", 3), ("a", 3)]);
assert_eq!(rle(""), Vec::<(char, usize)>::new());
```
