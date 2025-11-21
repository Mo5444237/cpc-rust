pub fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace().map(|s| s.parse::<i64>().unwrap());

    // n = number of segments, m = number of queries
    let n = iterator.next().unwrap_or(0) as usize;
    let m = iterator.next().unwrap_or(0) as usize;

    // Read segments
    let mut segments = Vec::with_capacity(n);
    let mut max_coord: usize = 0;

    for _ in 0..n {
        let left = iterator.next().unwrap() as usize;
        let right = iterator.next().unwrap() as usize;
        segments.push(Segment { left, right });

        if right > max_coord {
            max_coord = right;
        }
    }

    // Read queries
    let mut queries = Vec::with_capacity(m);

    for _ in 0..m {
        let left = iterator.next().unwrap() as usize;
        let right = iterator.next().unwrap() as usize;
        let k = iterator.next().unwrap();

        queries.push(Query { left, right, k });

        // Queries may refer to positions beyond the last segment endpoint.
        if right > max_coord {
            max_coord = right;
        }
    }

    // Edge case: if there is nothing at all, return empty string
    if max_coord == 0 && n == 0 && m == 0 {
        return String::new();
    }

    // Build difference array for coverage
    let len = max_coord + 1;
    let mut diff = vec![0_i64; len + 1];

    for seg in &segments {
        diff[seg.left] += 1;
        if seg.right + 1 < len {
            diff[seg.right + 1] -= 1;
        }
    }

    // Build coverage array via prefix sums
    let mut coverage = vec![0_i64; len];
    let mut running = 0_i64;
    for x in 0..len {
        running += diff[x];
        coverage[x] = running;
    }

    // Build segment tree on coverage
    let seg_tree = SegmentTree::new(&coverage);

    // Answer queries in order
    let mut result_lines = Vec::with_capacity(m);

    for query in queries {
        let exists = seg_tree.exists_value_in_range(query.left, query.right, query.k);
        if exists {
            result_lines.push("1".to_string());
        } else {
            result_lines.push("0".to_string());
        }
    }

    result_lines.join("\n")
}

#[derive(Debug, Clone, Copy)]
struct Segment {
    left: usize,
    right: usize,
}

#[derive(Debug, Clone, Copy)]
struct Query {
    left: usize,
    right: usize,
    k: i64,
}

#[derive(Debug, Clone, Copy)]
struct MinMaxNode {
    min_value: i64,
    max_value: i64,
}

impl MinMaxNode {
    fn from_single(value: i64) -> Self {
        Self {
            min_value: value,
            max_value: value,
        }
    }

    fn empty() -> Self {
        Self {
            min_value: i64::MAX,
            max_value: i64::MIN,
        }
    }

    fn merge(left: MinMaxNode, right: MinMaxNode) -> MinMaxNode {
        MinMaxNode {
            min_value: left.min_value.min(right.min_value),
            max_value: left.max_value.max(right.max_value),
        }
    }
}

struct SegmentTree {
    n: usize,
    tree: Vec<MinMaxNode>,
}

impl SegmentTree {
    fn new(values: &[i64]) -> Self {
        let n = values.len();
        let size = 4 * n.max(1);
        let mut st = Self {
            n,
            tree: vec![MinMaxNode::empty(); size],
        };

        if n > 0 {
            st.build(1, 0, n - 1, values);
        }

        st
    }

    fn build(&mut self, index: usize, left: usize, right: usize, values: &[i64]) {
        if left == right {
            self.tree[index] = MinMaxNode::from_single(values[left]);
            return;
        }

        let mid = (left + right) / 2;
        let left_child = index * 2;
        let right_child = left_child + 1;

        self.build(left_child, left, mid, values);
        self.build(right_child, mid + 1, right, values);

        self.tree[index] = MinMaxNode::merge(self.tree[left_child], self.tree[right_child]);
    }

    /// Returns true if there exists an index x in [ql, qr]
    /// such that coverage[x] == k.
    fn exists_value_in_range(&self, ql: usize, qr: usize, k: i64) -> bool {
        if self.n == 0 {
            return false;
        }
        self.exists_value_in_range_rec(1, 0, self.n - 1, ql, qr, k)
    }

    fn exists_value_in_range_rec(
        &self,
        index: usize,
        left: usize,
        right: usize,
        ql: usize,
        qr: usize,
        k: i64,
    ) -> bool {
        // No overlap
        if right < ql || qr < left {
            return false;
        }

        let node = self.tree[index];

        // Prune by min/max range: if k is outside [min_value, max_value],
        // it cannot exist in this segment.
        if k < node.min_value || k > node.max_value {
            return false;
        }

        // Leaf node: single position
        if left == right {
            // Here min_value == max_value == coverage[left]
            return node.min_value == k;
        }

        // Partial or full overlap, not a leaf: explore children
        let mid = (left + right) / 2;
        let left_child = index * 2;
        let right_child = left_child + 1;

        // Check left child first; if found, no need to check right.
        if self.exists_value_in_range_rec(left_child, left, mid, ql, qr, k) {
            return true;
        }

        // Otherwise, check right child.
        self.exists_value_in_range_rec(right_child, mid + 1, right, ql, qr, k)
    }
}
