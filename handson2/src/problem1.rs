pub fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace().map(|s| s.parse::<i64>().unwrap());

    let n = iterator.next().unwrap_or(0) as usize;
    let q = iterator.next().unwrap_or(0) as usize;

    let mut values = Vec::with_capacity(n);
    for _ in 0..n {
        values.push(iterator.next().unwrap());
    }

    let mut tree = SegmentTree::new(&values);
    let mut output = Vec::new();

    for _ in 0..q {
        let op_type = iterator.next().unwrap();

        match op_type {
            0 => {
                // Update(i, j, T): apply A[k] = min(A[k], T)
                let i = iterator.next().unwrap() as usize - 1;
                let j = iterator.next().unwrap() as usize - 1;
                let t = iterator.next().unwrap();
                tree.range_chmin(i, j, t);
            }
            1 => {
                // Max(i, j)
                let i = iterator.next().unwrap() as usize - 1;
                let j = iterator.next().unwrap() as usize - 1;
                let ans = tree.range_max(i, j);
                output.push(ans.to_string());
            }
            _ => panic!("Unknown operation type: {op_type}"),
        }
    }

    output.join("\n")
}

#[derive(Clone, Copy, Debug)]
struct Node {
    max_value: i64,
    second_max: i64,
}

// Node methods for Segment Tree Beats
impl Node {
    /// Create a node from a single value.
    fn from_single(value: i64) -> Self {
        Self {
            max_value: value,
            second_max: i64::MIN,
        }
    }

    /// Create an empty node (used for initialization and in special cases).
    fn empty() -> Self {
        Self {
            max_value: i64::MIN,
            second_max: i64::MIN,
        }
    }

    /// Merge two child nodes into a parent node.
    fn merge(left: Node, right: Node) -> Node {
        // If one node is empty, return the other
        if left.max_value == i64::MIN {
            return right;
        }
        if right.max_value == i64::MIN {
            return left;
        }

        // Both valid; compare max_value
        if left.max_value == right.max_value {
            Node {
                max_value: left.max_value,
                second_max: left.second_max.max(right.second_max),
            }
        } else if left.max_value > right.max_value {
            Node {
                max_value: left.max_value,
                second_max: left.second_max.max(right.max_value),
            }
        } else {
            Node {
                max_value: right.max_value,
                second_max: right.second_max.max(left.max_value),
            }
        }
    }
}

/// Segment Tree Beats implementation.
struct SegmentTree {
    n: usize,
    tree: Vec<Node>,
}

impl SegmentTree {
    fn new(values: &[i64]) -> Self {
        let n = values.len();
        let size = 4 * n.max(1);
        let mut st = Self {
            n,
            tree: vec![Node::empty(); size],
        };
        if n > 0 {
            st.build(1, 0, n - 1, values);
        }
        st
    }

    fn build(&mut self, index: usize, left: usize, right: usize, values: &[i64]) {
        // Leaf node
        if left == right {
            self.tree[index] = Node::from_single(values[left]);
            return;
        }

        let mid = (left + right) / 2;
        let left_child = index * 2;
        let right_child = left_child + 1;

        self.build(left_child, left, mid, values);
        self.build(right_child, mid + 1, right, values);

        self.tree[index] = Node::merge(self.tree[left_child], self.tree[right_child]);
    }

    /// Push the parent's max_value constraint down to children.
    fn push_down(&mut self, index: usize) {
        let parent_node = self.tree[index];
        let left_child = index * 2;
        let right_child = left_child + 1;

        // If child's maximum is too large, clamp it down
        if self.tree[left_child].max_value > parent_node.max_value {
            self.clamp_node(left_child, parent_node.max_value);
        }
        if self.tree[right_child].max_value > parent_node.max_value {
            self.clamp_node(right_child, parent_node.max_value);
        }
    }

    /// Apply chmin(x) to a node that satisfies: second_max < x < max_value.
    fn clamp_node(&mut self, index: usize, new_max: i64) {
        let node = &mut self.tree[index];
        if new_max >= node.max_value {
            return; // this would not change anything
        }
        node.max_value = new_max;
    }

    /// Wrapper for range chmin.
    fn range_chmin(&mut self, ql: usize, qr: usize, x: i64) {
        if self.n == 0 {
            return;
        }
        self.range_chmin_rec(1, 0, self.n - 1, ql, qr, x);
    }

    fn range_chmin_rec(
        &mut self,
        index: usize,
        left: usize,
        right: usize,
        ql: usize,
        qr: usize,
        x: i64,
    ) {
        let node = self.tree[index];

        // No overlap or nothing to reduce
        if right < ql || qr < left || node.max_value <= x {
            return;
        }

        // Fully covered and can apply clamp directly
        if ql <= left && right <= qr && node.second_max < x && x < node.max_value {
            self.clamp_node(index, x);
            return;
        }

        // Otherwise, push constraints down before recursion
        if left != right {
            self.push_down(index);
            let mid = (left + right) / 2;
            let left_child = index * 2;
            let right_child = left_child + 1;

            self.range_chmin_rec(left_child, left, mid, ql, qr, x);
            self.range_chmin_rec(right_child, mid + 1, right, ql, qr, x);

            self.tree[index] = Node::merge(self.tree[left_child], self.tree[right_child]);
        } else {
            // Leaf case — clamp it
            self.clamp_node(index, x);
        }
    }

    /// Wrapper for range max query.
    fn range_max(&mut self, ql: usize, qr: usize) -> i64 {
        if self.n == 0 {
            return i64::MIN;
        }
        self.range_max_rec(1, 0, self.n - 1, ql, qr)
    }

    fn range_max_rec(
        &mut self,
        index: usize,
        left: usize,
        right: usize,
        ql: usize,
        qr: usize,
    ) -> i64 {
        // No overlap
        if right < ql || qr < left {
            return i64::MIN;
        }
        // Fully covered
        if ql <= left && right <= qr {
            return self.tree[index].max_value;
        }

        // Partial overlap; push down and recurse
        self.push_down(index);

        let mid = (left + right) / 2;
        let left_child = index * 2;
        let right_child = left_child + 1;

        let max_left = self.range_max_rec(left_child, left, mid, ql, qr);
        let max_right = self.range_max_rec(right_child, mid + 1, right, ql, qr);
        max_left.max(max_right)
    }
}
