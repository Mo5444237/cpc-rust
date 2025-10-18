pub struct Node {
    key: u32,
    id_left: Option<usize>,
    id_right: Option<usize>,
}

impl Node {
    fn new(key: u32) -> Self {
        Self {
            key,
            id_left: None,
            id_right: None,
        }
    }
}

pub struct Tree {
    nodes: Vec<Node>,
}

impl Tree {
    pub fn with_root(key: u32) -> Self {
        Self {
            nodes: vec![Node::new(key)],
        }
    }

    /// Adds a child to the node with `parent_id` and returns the id of the new node.
    /// The new node has the specified `key`. The new node is the left  child of the  
    /// node `parent_id` iff `is_left` is `true`, the right child otherwise.
    ///
    /// # Panics
    /// Panics if the `parent_id` does not exist, or if the node `parent_id ` has  
    /// the child already set.
    pub fn add_node(&mut self, parent_id: usize, key: u32, is_left: bool) -> usize {
        assert!(
            parent_id < self.nodes.len(),
            "Parent node id does not exist"
        );
        if is_left {
            assert!(
                self.nodes[parent_id].id_left.is_none(),
                "Parent node has the left child already set"
            );
        } else {
            assert!(
                self.nodes[parent_id].id_right.is_none(),
                "Parent node has the right child already set"
            );
        }

        let child_id = self.nodes.len();
        self.nodes.push(Node::new(key));

        let child = if is_left {
            &mut self.nodes[parent_id].id_left
        } else {
            &mut self.nodes[parent_id].id_right
        };

        *child = Some(child_id);

        child_id
    }

    /// Returns the sum of all the keys in the tree
    pub fn sum(&self) -> u32 {
        self.rec_sum(Some(0))
    }

    /// A private recursive function that computes the sum of
    /// nodes in the subtree rooted at `node_id`.
    fn rec_sum(&self, node_id: Option<usize>) -> u32 {
        if let Some(id) = node_id {
            assert!(id < self.nodes.len(), "Node id is out of range");
            let node = &self.nodes[id];

            let sum_left = self.rec_sum(node.id_left);
            let sum_right = self.rec_sum(node.id_right);

            return sum_left + sum_right + node.key;
        }

        0
    }

    /// Returns `true` if the tree is a valid Binary Search Tree (BST), `false` otherwise.
    pub fn is_bst(&self) -> bool {
        self.rec_is_bst(Some(0), None, None)
    }

    /// A private recursive helper function that checks if the subtree rooted at
    /// `node_id` is a valid BST. The node's key must be greater than `min` and
    /// less than `max`.
    fn rec_is_bst(&self, node_id: Option<usize>, min: Option<u32>, max: Option<u32>) -> bool {
        // An empty subtree is a valid BST
        let Some(id) = node_id else {
            return true;
        };

        assert!(id < self.nodes.len(), "Node id is out of range");
        let node = &self.nodes[id];

        // Check if current node's key violates BST property
        if let Some(min_val) = min {
            if node.key <= min_val {
                return false;
            }
        }

        if let Some(max_val) = max {
            if node.key >= max_val {
                return false;
            }
        }

        // Recursively check left and right subtrees
        self.rec_is_bst(node.id_left, min, Some(node.key))
            && self.rec_is_bst(node.id_right, Some(node.key), max)
    }

    /// Returns the maximum sum of a simple path connecting two leaves.
    /// A simple path is a path that doesn't visit any node more than once.
    ///
    /// # Panics
    /// Panics if the tree doesn't have at least two leaves (cannot form a path between two leaves).
    pub fn max_path_sum(&self) -> u32 {
        let leaf_count = self.count_leaves(Some(0));
        assert!(
            leaf_count >= 2,
            "Tree must have at least 2 leaves to form a path between leaves (found {leaf_count} leaf/leaves)"
        );

        let mut max_sum = 0;
        self.rec_max_path_sum(Some(0), &mut max_sum);
        max_sum
    }

    /// Counts the number of leaves in the subtree rooted at `node_id`.
    fn count_leaves(&self, node_id: Option<usize>) -> usize {
        let Some(id) = node_id else {
            return 0;
        };

        assert!(id < self.nodes.len(), "Node id is out of range");
        let node = &self.nodes[id];

        // A leaf node has no children
        if node.id_left.is_none() && node.id_right.is_none() {
            return 1;
        }

        // Count leaves in both subtrees
        self.count_leaves(node.id_left) + self.count_leaves(node.id_right)
    }

    /// A private recursive helper that computes the maximum path sum.
    /// Returns the maximum sum from the current node down to any leaf.
    /// Updates `max_sum` with the maximum path sum passing through this node.
    fn rec_max_path_sum(&self, node_id: Option<usize>, max_sum: &mut u32) -> Option<u32> {
        let id = node_id?;

        assert!(id < self.nodes.len(), "Node id is out of range");
        let node = &self.nodes[id];

        // Recursively get max path sums for left and right subtrees
        let left_max = self.rec_max_path_sum(node.id_left, max_sum);
        let right_max = self.rec_max_path_sum(node.id_right, max_sum);

        match (left_max, right_max) {
            (Some(left), Some(right)) => {
                // Internal node with both children: path through this node
                let path_through_node = left + node.key + right;
                *max_sum = (*max_sum).max(path_through_node);

                // Return max path going down from this node
                Some(node.key + left.max(right))
            }
            (Some(left), None) => Some(node.key + left),
            (None, Some(right)) => Some(node.key + right),
            (None, None) => Some(node.key),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        let mut tree = Tree::with_root(10);
        assert_eq!(tree.sum(), 10);

        tree.add_node(0, 5, true); // id 1
        tree.add_node(0, 22, false); // id 2
        assert_eq!(tree.sum(), 37);

        tree.add_node(1, 7, false); // id 3
        tree.add_node(2, 20, true); // id 4
        assert_eq!(tree.sum(), 64);
    }

    // is_bst tests
    #[test]
    fn test_is_bst_valid() {
        // Valid BST:
        //       10
        //      /  \
        //     5    15
        //    / \   /
        //   3   7 12
        let mut tree = Tree::with_root(10);
        tree.add_node(0, 5, true);
        tree.add_node(0, 15, false);
        tree.add_node(1, 3, true);
        tree.add_node(1, 7, false);
        tree.add_node(2, 12, true);

        assert!(tree.is_bst());
    }

    #[test]
    fn test_is_bst_invalid_subtree() {
        // Invalid: 12 is in left subtree but > root (10)
        //       10
        //      /  \
        //     5    15
        //      \
        //      12
        let mut tree = Tree::with_root(10);
        tree.add_node(0, 5, true);
        tree.add_node(0, 15, false);
        tree.add_node(1, 12, false);

        assert!(!tree.is_bst());
    }

    #[test]
    fn test_is_bst_equal_values() {
        // Invalid: equal values not allowed
        let mut tree = Tree::with_root(10);
        tree.add_node(0, 10, true);
        tree.add_node(0, 15, false);

        assert!(!tree.is_bst());
    }

    // max_path_sum tests
    #[test]
    fn test_max_path_sum_simple() {
        //     10
        //    /  \
        //   5    15
        // Max path: 5 -> 10 -> 15 = 30
        let mut tree = Tree::with_root(10);
        tree.add_node(0, 5, true);
        tree.add_node(0, 15, false);

        assert_eq!(tree.max_path_sum(), 30);
    }

    #[test]
    fn test_max_path_sum_not_through_root() {
        // Max path doesn't go through root
        //         1
        //        / \
        //       50  2
        //      / \   \
        //     30 40   3
        // Max path: 30 -> 50 -> 40 = 120
        let mut tree = Tree::with_root(1);
        tree.add_node(0, 50, true);
        tree.add_node(0, 2, false);
        tree.add_node(1, 30, true);
        tree.add_node(1, 40, false);
        tree.add_node(2, 3, false);

        assert_eq!(tree.max_path_sum(), 120);
    }

    #[test]
    #[should_panic(expected = "Tree must have at least 2 leaves")]
    fn test_max_path_sum_single_leaf() {
        // Should panic with only 1 leaf
        let mut tree = Tree::with_root(10);
        tree.add_node(0, 5, true);
        tree.max_path_sum();
    }
}
