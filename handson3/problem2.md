# HandsOn 3 — Problem 2: Design a Course

## Problem Overview

A professor needs to design a new course by selecting topics from a list of candidates. Each topic has two attributes: **beauty** (how interesting it is) and **difficulty** (how hard it is).

**Key Constraints:**
* You have **n topics** to choose from
* Each topic has a **beauty** value and a **difficulty** value
* Selected topics must have **strictly increasing beauty**: beauty[i₁] < beauty[i₂] < beauty[i₃] < ...
* Selected topics must have **strictly increasing difficulty**: difficulty[i₁] < difficulty[i₂] < difficulty[i₃] < ...

**Goal:** Select the maximum number of topics that satisfy both constraints.

**Example:**
```
Input:
5
0 3     // Topic 0: beauty=0, difficulty=3
99 1    // Topic 1: beauty=99, difficulty=1
11 20   // Topic 2: beauty=11, difficulty=20
1 2     // Topic 3: beauty=1, difficulty=2
10 5    // Topic 4: beauty=10, difficulty=5

Output:
3       // Select topics [0, 4, 2]: beauty: 0→10→11, difficulty: 3→5→20
```

## Key Challenge

This is a **2D constraint optimization problem**:
* We must satisfy TWO conditions simultaneously (beauty AND difficulty increasing)
* The order of topics in the input doesn't matter
* We can select topics in any order, but the selected sequence must satisfy both constraints

Naive approaches won't work:
* **Greedy** (pick lowest difficulty first or highest beauty first) fails because we need to balance both dimensions
* **Brute force** (try all subsequences) would be O(2^n), which is exponential

We need a clever combination of **sorting + dynamic programming** to solve this efficiently.

## High-Level Idea — Sort + Longest Increasing Subsequence

The key insight is to reduce the 2D problem to a 1D problem:

1. **Sort topics by beauty** (ascending) → This ensures the beauty constraint is automatically satisfied when we select topics in sorted order
2. **Find the Longest Increasing Subsequence (LIS)** of difficulty values → This ensures the difficulty constraint is satisfied
3. The length of the LIS is our answer

**Why does this work?**
* After sorting by beauty, if we select topics at positions i₁ < i₂ < i₃ from the sorted array, we automatically get beauty[i₁] ≤ beauty[i₂] ≤ beauty[i₃]
* By finding LIS on difficulties, we ensure difficulty[i₁] < difficulty[i₂] < difficulty[i₃]
* Combined, both constraints are satisfied!

## Critical Detail — Handling Ties

**Problem:** If two topics have the same beauty, we cannot select both (strictly increasing requirement).

**Solution:** When sorting, use a **secondary sort key**:
```
Sort by: (beauty ascending, difficulty descending)
```

**Why descending for difficulty?**

Consider topics with the same beauty:
```
Topic A: beauty=5, difficulty=10
Topic B: beauty=5, difficulty=20
```

If we sort difficulty ascending: `[5,10], [5,20]`
* LIS might find both: `[10, 20]` with length 2
* But we **cannot** select both because they have the same beauty!

If we sort difficulty descending: `[5,20], [5,10]`
* LIS can only find one of them: length 1
* This prevents selecting topics with the same beauty

## Algorithm Walkthrough

### Step 1: Parse Input
```rust
let n = 5;
let topics = vec![
    (0, 3),   // beauty=0, difficulty=3
    (99, 1),  // beauty=99, difficulty=1
    (11, 20), // beauty=11, difficulty=20
    (1, 2),   // beauty=1, difficulty=2
    (10, 5),  // beauty=10, difficulty=5
];
```

### Step 2: Sort Topics

**Sorting rule:** `(beauty ↑, difficulty ↓)`

| Original | Beauty | Difficulty | After Sort |
|----------|--------|------------|------------|
| Topic 0  | 0      | 3          | Position 0 |
| Topic 3  | 1      | 2          | Position 1 |
| Topic 4  | 10     | 5          | Position 2 |
| Topic 2  | 11     | 20         | Position 3 |
| Topic 1  | 99     | 1          | Position 4 |

**Sorted topics:**
```rust
topics = [(0, 3), (1, 2), (10, 5), (11, 20), (99, 1)]
```

### Step 3: Extract Difficulties
```rust
difficulties = [3, 2, 5, 20, 1]
```

### Step 4: Find LIS of Difficulties

We use an efficient O(n log n) algorithm with binary search:

**Algorithm maintains:** `tails[i]` = smallest tail element of all increasing subsequences of length `i+1`

```
Process difficulty=3:
  tails = []
  Binary search: position 0
  Insert at position 0
  tails = [3]

Process difficulty=2:
  tails = [3]
  Binary search: 2 < 3, position 0
  Replace tails[0] = 2  (found a better tail for length-1 subsequences)
  tails = [2]

Process difficulty=5:
  tails = [2]
  Binary search: 5 > 2, position 1
  Append (extend the subsequence)
  tails = [2, 5]

Process difficulty=20:
  tails = [2, 5]
  Binary search: 20 > 5, position 2
  Append (extend the subsequence)
  tails = [2, 5, 20]

Process difficulty=1:
  tails = [2, 5, 20]
  Binary search: 1 < 2, position 0
  Replace tails[0] = 1  (found an even better tail)
  tails = [1, 5, 20]

Result: len(tails) = 3
```

**Answer:** 3 topics can be selected

**The actual sequence:**
* Difficulties: `[2, 5, 20]` (or `[3, 5, 20]`)
* Corresponds to topics: [Topic 3, Topic 4, Topic 2]
* Beauty: 1 → 10 → 11  (strictly increasing)
* Difficulty: 2 → 5 → 20  (strictly increasing)

## Longest Increasing Subsequence (LIS) — Detailed Explanation

The LIS algorithm uses **dynamic programming with binary search** for efficiency.

**Key Idea:**
* Maintain array `tails` where `tails[i]` = smallest tail of all increasing subsequences of length `i+1`
* For each new element, use binary search to find where it fits
* Either extend the longest subsequence or improve a shorter one

**Why does this work?**
* `tails` always remains sorted (invariant)
* If we can't extend the longest subsequence, we improve a shorter one by replacing its tail with a smaller value
* This gives us more room for future extensions

**Complexity:**
* Binary search: O(log n) per element
* n elements total
* **Total: O(n log n)**

## Implementation
```rust
pub fn solve(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();

    let n: usize = lines[0].trim().parse().unwrap();

    if n == 0 {
        return "0".to_string();
    }

    // Read topics (beauty, difficulty)
    let mut topics = Vec::with_capacity(n);
    for line in lines.iter().skip(1).take(n) {
        let mut parts = line.split_whitespace();
        let beauty: i32 = parts.next().unwrap().parse().unwrap();
        let difficulty: i32 = parts.next().unwrap().parse().unwrap();
        topics.push((beauty, difficulty));
    }

    // Sort by beauty ascending, then difficulty descending
    topics.sort_by(|a, b| {
        if a.0 == b.0 {
            b.1.cmp(&a.1) // difficulty descending when beauty is equal
        } else {
            a.0.cmp(&b.0) // beauty ascending
        }
    });

    // Extract difficulties after sorting
    let difficulties: Vec<i32> = topics.iter().map(|&(_, diff)| diff).collect();

    // Find LIS of difficulties
    let result = lis_length(&difficulties);

    result.to_string()
}

/// Find the length of the Longest Increasing Subsequence (strictly increasing)
fn lis_length(arr: &[i32]) -> usize {
    if arr.is_empty() {
        return 0;
    }

    let mut tails = Vec::new();

    for &num in arr {
        // Binary search for the position where num should be inserted
        let pos = tails.binary_search(&num).unwrap_or_else(|pos| pos);

        if pos == tails.len() {
            tails.push(num);
        } else {
            tails[pos] = num;
        }
    }

    tails.len()
}
```

## Complexity Analysis

### Time Complexity: O(n log n)

* **Parsing:** O(n) — reading n topics
* **Sorting:** O(n log n) — standard comparison sort
* **Extracting difficulties:** O(n) — single pass through sorted topics
* **LIS with binary search:** O(n log n)
  - n iterations through the array
  - Each iteration: O(log n) binary search
* **Total:** O(n + n log n + n + n log n) = **O(n log n)** 

### Space Complexity: O(n)

* `topics` vector: O(n)
* `difficulties` vector: O(n)
* `tails` vector in LIS: O(n) worst case
* **Total:** **O(n)** 
