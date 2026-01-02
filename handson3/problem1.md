# HandsOn 3 — Problem 1: Holiday Planning

## Problem Overview

You're planning a Christmas vacation across Europe with a limited number of days before your exam. Each city has a tour guide with a daily itinerary showing how many attractions you can visit each day.

**Key Constraints:**
* You have **n cities** to choose from
* You have **D total days** for vacation
* Each city has a daily itinerary: `attractions[day]` = number of attractions on that day
* **You must visit attractions in order** — if you spend `k` days in a city, you visit attractions from days 1 through k (no cherry-picking)

**Goal:** Maximize the total number of attractions visited across all cities.

**Example:**
```
Input:
2 3      // n=2 cities, D=3 days
3 2 1    // Florence: 3 attractions on day 1, 2 on day 2, 1 on day 3
3 1 1    // London: 3 attractions on day 1, 1 on day 2, 1 on day 3

Output:
8        // Spend 2 days in Florence (3+2=5) and 1 day in London (3)
```

## Key Challenge

This is a **resource allocation problem**:
* We have a limited resource (days)
* We must distribute this resource across multiple options (cities)
* Each option has diminishing returns (attractions per additional day may vary)

Naive approaches won't work:
* **Greedy** (always pick the city with most attractions per day) fails because it doesn't consider the global optimal allocation
* **Brute force** (try all possible day distributions) would be O(D^n), which is exponential

We need **Dynamic Programming** to find the optimal allocation efficiently.

## High-Level Idea — Dynamic Programming

The core insight is to build solutions incrementally:

1. **Start with no cities and no days** → 0 attractions
2. **Add cities one by one**, deciding for each city how many days to spend there
3. **Build on previous solutions** to avoid recomputing

**DP State Definition:**
```
dp[i][j] = maximum attractions achievable using the first i cities with exactly j days
```

**Base Case:**
```
dp[0][j] = 0 for all j  (no cities = no attractions)
```

**Transition:**

For each city `i` and day budget `j`, try spending `k` days in city `i` (where 0 ≤ k ≤ j):
```
dp[i][j] = max over all k of: dp[i-1][j-k] + attractions_in_city_i_for_k_days
```

This means:
* Use `j-k` days optimally in the first `i-1` cities → that's `dp[i-1][j-k]`
* Spend `k` days in city `i` → that gives us some attractions
* Take the maximum over all possible values of `k`

## Precomputation — Cumulative Attractions

Before running the DP, we precompute cumulative attractions for each city:
```
cum[i][d] = total attractions when spending exactly d days in city i
```

**Example:** Florence with itinerary `[3, 2, 1]`
```
cum[Florence][0] = 0           (0 days)
cum[Florence][1] = 3           (day 1: 3 attractions)
cum[Florence][2] = 3 + 2 = 5   (days 1-2: 5 attractions)
cum[Florence][3] = 3 + 2 + 1 = 6  (days 1-3: 6 attractions)
```

This precomputation allows us to look up "attractions for k days in city i" in O(1) time during the DP.

## Algorithm Walkthrough

### Step 1: Parse Input
```rust
let n = 2;  // number of cities
let d = 3;  // total days
let cities = vec![
    vec![3, 2, 1],  // Florence
    vec![3, 1, 1],  // London
];
```

### Step 2: Compute Cumulative Attractions
```rust
cum[0] = [0, 0, 0, 0]           // No city 0
cum[1] = [0, 3, 5, 6]           // Florence
cum[2] = [0, 3, 4, 5]           // London
```

### Step 3: Initialize DP Table
```rust
dp[0][0] = 0, dp[0][1] = 0, dp[0][2] = 0, dp[0][3] = 0
```

### Step 4: Fill DP Table

**For city 1 (Florence):**
```
dp[1][0]:
  k=0: dp[0][0] + cum[1][0] = 0 + 0 = 0
  → dp[1][0] = 0

dp[1][1]:
  k=0: dp[0][1] + cum[1][0] = 0 + 0 = 0
  k=1: dp[0][0] + cum[1][1] = 0 + 3 = 3
  → dp[1][1] = 3

dp[1][2]:
  k=0: dp[0][2] + cum[1][0] = 0 + 0 = 0
  k=1: dp[0][1] + cum[1][1] = 0 + 3 = 3
  k=2: dp[0][0] + cum[1][2] = 0 + 5 = 5
  → dp[1][2] = 5

dp[1][3]:
  k=0,1,2,3: max = 6
  → dp[1][3] = 6
```

**For city 2 (Florence + London):**
```
dp[2][0] = 0  (no days = no attractions)

dp[2][1]:
  k=0: dp[1][1] + cum[2][0] = 3 + 0 = 3  (1 day Florence, 0 London)
  k=1: dp[1][0] + cum[2][1] = 0 + 3 = 3  (0 day Florence, 1 London)
  → dp[2][1] = 3

dp[2][2]:
  k=0: dp[1][2] + cum[2][0] = 5 + 0 = 5  (2 days Florence, 0 London)
  k=1: dp[1][1] + cum[2][1] = 3 + 3 = 6  (1 day Florence, 1 London)
  k=2: dp[1][0] + cum[2][2] = 0 + 4 = 4  (0 days Florence, 2 London)
  → dp[2][2] = 6

dp[2][3]:
  k=0: dp[1][3] + cum[2][0] = 6 + 0 = 6  (3 days Florence, 0 London)
  k=1: dp[1][2] + cum[2][1] = 5 + 3 = 8  (2 days Florence, 1 London) ✓
  k=2: dp[1][1] + cum[2][2] = 3 + 4 = 7  (1 day Florence, 2 London)
  k=3: dp[1][0] + cum[2][3] = 0 + 5 = 5  (0 days Florence, 3 London)
  → dp[2][3] = 8
```

### Final DP Table

| City | 0 days | 1 day | 2 days | 3 days |
|------|--------|-------|--------|--------|
| 0    | 0      | 0     | 0      | 0      |
| 1    | 0      | 3     | 5      | 6      |
| 2    | 0      | 3     | 6      | **8**  |

**Answer:** `dp[2][3] = 8` attractions

**Optimal allocation:** 2 days in Florence (5 attractions) + 1 day in London (3 attractions) = 8 total

## Implementation
```rust
pub fn solve(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();

    let mut first_line = lines[0].split_whitespace();
    let n = first_line.next().unwrap().parse::<usize>().unwrap();
    let d = first_line.next().unwrap().parse::<usize>().unwrap();

    // Read cities' attractions
    let mut cities = Vec::with_capacity(n);
    for line in lines.iter().skip(1).take(n) {
        let attractions: Vec<usize> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        cities.push(attractions);
    }

    // Precompute cumulative attractions
    let mut cum = vec![vec![0; d + 1]; n + 1];
    for i in 1..=n {
        let city = &cities[i - 1];
        for day in 1..=d {
            cum[i][day] = if day <= city.len() {
                cum[i][day - 1] + city[day - 1]
            } else {
                cum[i][day - 1]
            };
        }
    }

    // dp[i][j] = max attractions using first i cities with j days
    let mut dp = vec![vec![0; d + 1]; n + 1];

    for i in 1..=n {
        for j in 0..=d {
            for k in 0..=j {
                dp[i][j] = dp[i][j].max(dp[i - 1][j - k] + cum[i][k]);
            }
        }
    }

    dp[n][d].to_string()
}
```

### Code Breakdown

1. **Parse input** (lines 1-12):
   - Extract n cities and d days
   - Read each city's daily attractions into a vector

2. **Precompute cumulative sums** (lines 14-24):
   - `cum[i][day]` = total attractions for spending `day` days in city `i`
   - Handle cases where itinerary is shorter than d days

3. **Dynamic Programming** (lines 26-34):
   - Initialize dp table with zeros
   - For each city and day count, try all possible allocations
   - Take maximum over all choices

4. **Return result** (line 36):
   - `dp[n][d]` contains the optimal solution

## Complexity Analysis

### Time Complexity: O(n × D²)

* **Precomputation:** O(n × D) — for each of n cities, compute D cumulative sums
* **DP:** O(n × D²) — three nested loops:
  - n cities
  - D possible day counts
  - Up to D choices for k
* **Total:** O(n × D + n × D²) = **O(n × D²)**

### Space Complexity: O(n × D)

* `cum` table: (n+1) × (D+1) = O(n × D)
* `dp` table: (n+1) × (D+1) = O(n × D)
* `cities` vector: O(total attractions listed) ≈ O(n × D) in worst case
* **Total:** **O(n × D)**
