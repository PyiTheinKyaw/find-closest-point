use std::fmt::Debug;
use crate::model::direction::NodeDirection;
#[derive(Debug)]
pub struct Node<T> {
    index: usize,
    values: Option<Vec<T>>,
    is_leaf: bool,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T> Node<T>
{
    pub fn get_empty_node(value: T,  depth: usize) -> Node<T> {
        Node {
            index: 0,
            values: None,
            is_leaf: false,
            left: None,
            right: None
        }
    }

    pub fn set_child_node(&mut self, node: Self, direction: NodeDirection) {
        if direction == NodeDirection::LEFT {
            self.left = Some(Box::new(node));
        }
        else if direction == NodeDirection::RIGHT {
            self.right = Some(Box::new(node));
        }
    }

    /// Creates a leaf node for a tree with the given index and values.
    ///
    /// A leaf node represents a terminal node in a tree and holds a collection of values.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the node within the tree structure.
    /// * `values` - A vector containing the values associated with the leaf node.
    ///
    /// # Returns
    ///
    /// A newly created leaf node with the specified index and values.
    ///
    /// # Examples
    ///
    /// ```
    /// use fnp::model::node::Node;
    ///
    /// // Create a leaf node with index 0 and values [1, 2, 3]
    /// let leaf_node = Node::<i32>::create_leaf_node(0, vec![1, 2, 3]);
    /// ```
    ///
    /// # Notes
    ///
    /// - The `is_leaf` field is set to `true` to indicate that this node is a leaf.
    /// - The `left` and `right` fields are set to `None` since leaf nodes do not have children.
    ///
    /// Leaf nodes are typically used in tree structures to represent endpoints or data storage points.
    pub fn create_leaf_node(index: usize, values: Vec<T>) -> Self{
        Self {
            index,
            values: Some(values),
            is_leaf: true,
            left: None,
            right: None
        }
    }


    /// Creates an internal node for a tree with the given index and child nodes.
    ///
    /// An internal node represents a non-terminal node in a tree and holds references to its child nodes.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the node within the tree structure.
    /// * `left` - An optional box containing the left child node of the internal node.
    /// * `right` - An optional box containing the right child node of the internal node.
    ///
    /// # Returns
    ///
    /// A newly created internal node with the specified index and child nodes.
    ///
    /// # Examples
    ///
    /// ```
    /// use fnp::model::node::Node;
    ///
    /// // Create an internal node with index 1 and left and right child nodes (In this case, it will be none)
    /// let internal_node = Node::<i32>::create_internal_node(1, None, None);
    /// ```
    ///
    /// # Notes
    ///
    /// - The `is_leaf` field is set to `false` to indicate that this node is not a leaf.
    /// - The `values` field is set to `None` since internal nodes do not hold values directly.
    ///
    /// Internal nodes are used to represent intermediate points in tree structures, facilitating traversal and organization.
    pub fn create_internal_node(index: usize, left: Option<Box<Node<T>>>, right: Option<Box<Node<T>>>) -> Self{
        Self {
            index,
            values: None,
            is_leaf: false,
            left,
            right
        }
    }
}