# HandsOn 1 — Binary Tree Traversals

**Source:** [Professor Rossano Venturini's HandsOn 1](https://pages.di.unipi.it/rossano/blog/2024/handson12425/)

**Tasks.** Implement recursive methods for binary tree operations:

1. **is_bst()** — Check if the tree is a valid Binary Search Tree
2. **max_path_sum()** — Find the maximum sum path between two leaves

## Tree Structure

Uses a vector-based implementation where nodes are stored in `Vec<Node>` and referenced by indices:

```rust
pub struct Node {
    key: u32,
    id_left: Option<usize>,
    id_right: Option<usize>,
}

pub struct Tree {
    nodes: Vec<Node>,
}
```


## Exercise #1 — Binary Search Tree Validation

**Task.** Implement

```rust
pub fn is_bst(&self) -> bool
```

Return `true` if the tree satisfies the BST property: for every node, all values in its left subtree are **strictly less** than the node's key, and all values in its right subtree are **strictly greater**.

**Examples**

Valid BST:
```
       10
      /  \
     5    15
    / \   /
   3   7 12
```
→ `true`

Invalid BST (12 > 10 but in left subtree):
```
       10
      /  \
     5    15
      \
      12
```
→ `false`


## Possible Solutions & Complexities

### 1) Min/Max Bounds Propagation — **Chosen**

* **Idea:** Track valid range `(min, max)` for each node as we traverse. Each node must satisfy `min < key < max`. For left children, update `max = parent.key`; for right children, update `min = parent.key`.
* **Time:** `O(n)` — single pass visiting each node once
* **Space:** `O(h)` — recursion depth equals tree height
* **Pros:** Catches violations at any depth; no need to traverse each node multiple times.

### 2) In-order Traversal

* Perform in-order traversal and verify the sequence is strictly increasing.
* **Time:** `O(n)`
* **Space:** `O(n)` for storing values or `O(h)` if checking on-the-fly
* **Note:** Works but requires additional state management; bounds method is more direct.

### 3) Naive Per-Node Check

* For each node, verify all left descendants < node < all right descendants.
* **Time:** `O(n²)` — too slow
* **Space:** `O(h)`


## Why This Solution

The min/max bounds approach elegantly propagates constraints down the tree. It directly encodes the BST invariant and catches subtle violations (like a node in the left subtree being greater than an ancestor) in a single pass.


## Rust Implementation

```rust
pub fn is_bst(&self) -> bool {
    self.rec_is_bst(Some(0), None, None)
}

fn rec_is_bst(&self, node_id: Option<usize>, min: Option<u32>, max: Option<u32>) -> bool {
    let Some(id) = node_id else {
        return true;  // empty subtree is valid
    };

    let node = &self.nodes[id];

    // Check bounds
    if let Some(min_val) = min {
        if node.key <= min_val { return false; }
    }
    if let Some(max_val) = max {
        if node.key >= max_val { return false; }
    }

    // Recurse: left inherits max, right inherits min
    self.rec_is_bst(node.id_left, min, Some(node.key))
        && self.rec_is_bst(node.id_right, Some(node.key), max)
}
```

### How it works (step by step)

* Start at root with no constraints: `min = None`, `max = None`.
* For each node:
  * Verify `min < node.key < max` (strict inequalities).
  * Pass `(min, node.key)` to right subtree (all values must be > node).
  * Pass `(node.key, max)` to left subtree (all values must be < node).
* Empty subtrees return `true` (base case).


## Correctness & Complexity

* **Correctness:** The algorithm maintains the invariant that each node's key lies strictly between its inherited bounds. This precisely captures the BST property for all nodes.
* **Time:** `O(n)` — each node visited once
* **Space:** `O(h)` — recursion stack depth = tree height


## Tests (subset)

```rust
// Valid BST
let mut tree = Tree::with_root(10);
tree.add_node(0, 5, true);
tree.add_node(0, 15, false);
tree.add_node(1, 3, true);
tree.add_node(1, 7, false);
tree.add_node(2, 12, true);
assert!(tree.is_bst());  // true

// Invalid: subtree violation
let mut tree = Tree::with_root(10);
tree.add_node(0, 5, true);
tree.add_node(0, 15, false);
tree.add_node(1, 12, false);  // 12 > 10 but in left subtree!
assert!(!tree.is_bst());  // false

// Invalid: equal values
let mut tree = Tree::with_root(10);
tree.add_node(0, 10, true);
assert!(!tree.is_bst());  // false
```


## Exercise #2 — Maximum Path Sum Between Leaves

**Task.** Implement

```rust
pub fn max_path_sum(&self) -> u32
```

Return the maximum sum of a **simple path** connecting **two leaves**. A simple path visits each node at most once.

**Panics** if the tree has fewer than 2 leaves (no valid leaf-to-leaf path exists).

**Examples**

```
       10
      /  \
     5    15
```
→ `5 + 10 + 15 = 30`

Path not through root:
```
         1
        / \
       50  2
      / \   \
     30 40   3
```
→ `30 + 50 + 40 = 120` (max path doesn't include root!)


## Possible Solutions & Complexities

### 1) Recursive with Global Maximum Tracking — **Chosen**

* **Idea:** For each node, compute:
  1. **Return value:** Maximum path sum from this node **down** to any leaf (for parent to use).
  2. **Side effect:** Update global max if path **through** this node (connecting left and right subtrees) is better.
* **Time:** `O(n)` — visit each node once
* **Space:** `O(h)` — recursion depth
* **Pros:** Finds paths not going through root; elegant use of mutable reference to track global state.

### 2) DFS with All Leaf Paths

* Find all leaf-to-leaf paths and compute their sums.
* **Time:** `O(n²)` or worse — exponential number of paths in degenerate cases
* **Space:** `O(n * h)` for storing paths
* **Note:** Overkill; we only need the maximum.

### 3) Dynamic Programming on Tree

* Bottom-up DP computing max paths for each subtree.
* **Time:** `O(n)`
* **Space:** `O(n)` for DP table
* **Note:** Works but more complex; recursive approach with mutable max is cleaner.


## Why This Solution

The recursive approach elegantly handles the dual nature of tree paths: at each node, we must decide whether to **extend** a path upward (return best child path) or **complete** a path (connect left + node + right). Using a mutable reference for the global maximum allows all recursive calls to cooperate naturally.


## Rust Implementation

```rust
pub fn max_path_sum(&self) -> u32 {
    let leaf_count = self.count_leaves(Some(0));
    assert!(
        leaf_count >= 2,
        "Tree must have at least 2 leaves (found {leaf_count})"
    );

    let mut max_sum = 0;
    self.rec_max_path_sum(Some(0), &mut max_sum);
    max_sum
}

fn rec_max_path_sum(&self, node_id: Option<usize>, max_sum: &mut u32) -> Option<u32> {
    let id = node_id?;
    let node = &self.nodes[id];

    let left_max = self.rec_max_path_sum(node.id_left, max_sum);
    let right_max = self.rec_max_path_sum(node.id_right, max_sum);

    match (left_max, right_max) {
        (Some(left), Some(right)) => {
            // Path through this node: left -> node -> right
            let path_through = left + node.key + right;
            *max_sum = (*max_sum).max(path_through);

            // Return best downward path for parent
            Some(node.key + left.max(right))
        }
        (Some(left), None) => Some(node.key + left),
        (None, Some(right)) => Some(node.key + right),
        (None, None) => Some(node.key),  // leaf node
    }
}

fn count_leaves(&self, node_id: Option<usize>) -> usize {
    let Some(id) = node_id else { return 0; };
    let node = &self.nodes[id];

    if node.id_left.is_none() && node.id_right.is_none() {
        return 1;  // leaf
    }

    self.count_leaves(node.id_left) + self.count_leaves(node.id_right)
}
```

### How it works (step by step)

1. **Validate:** Ensure tree has ≥ 2 leaves (needed for leaf-to-leaf path).
2. **Recursive computation:** For each node:
   * Compute best paths from left and right children.
   * If node has **both children**: compute path **through** node (`left + node + right`) and update global max.
   * Return best **downward** path (`node + max(left, right)`) for parent to extend.
3. **Pattern match cases:**
   * Both children → consider path through node, return best downward path
   * One child → extend that child's path with current node
   * Leaf → return node's value


## Correctness & Complexity

* **Correctness:** At each node with two children, we consider all paths passing through it. By tracking the global maximum across all such paths, we find the optimal leaf-to-leaf path.
* **Time:** `O(n)` — each node visited once
* **Space:** `O(h)` — recursion stack depth


## Key Insight

A path can **turn** at any internal node (connecting left and right subtrees). When going **upward** to a parent, we can only bring **one** branch. The mutable `max_sum` lets any node update the answer if it finds a better path.


## Tests (subset)

```rust
// Simple path through root
let mut tree = Tree::with_root(10);
tree.add_node(0, 5, true);
tree.add_node(0, 15, false);
assert_eq!(tree.max_path_sum(), 30);  // 5 + 10 + 15

// Path NOT through root (critical test!)
let mut tree = Tree::with_root(1);
tree.add_node(0, 50, true);
tree.add_node(0, 2, false);
tree.add_node(1, 30, true);
tree.add_node(1, 40, false);
tree.add_node(2, 3, false);
assert_eq!(tree.max_path_sum(), 120);  // 30 + 50 + 40

// Should panic: only 1 leaf
let mut tree = Tree::with_root(10);
tree.add_node(0, 5, true);  // chain: only one leaf
tree.max_path_sum();  // panics!
```


## Overall Complexity Summary

| Method          | Time  | Space | Description                                    |
| --------------- | ----- | ----- | ---------------------------------------------- |
| `is_bst()`      | O(n)  | O(h)  | Min/max bounds propagation                     |
| `max_path_sum()`| O(n)  | O(h)  | Recursive with global max + leaf-to-leaf logic |

Both solutions are optimal for tree traversal problems!
