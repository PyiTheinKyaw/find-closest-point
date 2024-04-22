use std::fmt::Debug;
use crate::model::direction::NodeDirection;
#[derive(Debug)]
pub struct Node<T> {
    pub index: f32,
    pub values: Option<Vec<T>>,
    pub is_leaf: bool,
    pub left: Option<Box<Node<T>>>,
    pub right: Option<Box<Node<T>>>,
}

impl<T> Node<T>
{
    /// Constructs an empty node with default values.
    /// 
    /// An empty node is created with default values for its fields, representing a placeholder node
    /// that can be populated later with actual data or child nodes.
    /// 
    /// # Returns
    /// 
    /// An empty node with default values initialized for its fields.
    ///
    /// # Examples
    /// 
    /// ```
    /// use fnp::model::node::Node;
    /// 
    /// // Get an empty node
    /// let empty_node = Node::<i32>::get_empty_node();
    /// ```
    /// 
    /// # Notes
    /// 
    /// - The `index` field is set to `0` by default.
    /// - The `values` field is set to `None` since the node does not hold any values initially.
    /// - The `is_leaf`, `left`, and `right` fields are set to `false`, `None`, and `None` respectively,
    ///   indicating that the node is not a leaf and does not have any child nodes.
    ///
    /// Empty nodes serve as placeholders in tree structures, providing a starting point for building the tree.
    pub fn get_empty_node() -> Node<T> {
        Node {
            index: 0.0,
            values: None,
            is_leaf: false,
            left: None,
            right: None
        }
    }

    /// Sets the child node of the current node in the specified direction.
    /// 
    /// This method updates the child node of the current node based on the specified direction,
    /// either left or right, by replacing the existing child node with the provided node.
    /// 
    /// # Arguments
    /// 
    /// * `node` - The node to be set as the child node.
    /// * `index` - The index of the child node within the tree structure.
    /// * `direction` - The direction in which to set the child node (left or right).
    /// 
    /// # Examples
    /// 
    /// ```
    /// use fnp::model::node::Node;
    /// use fnp::model::direction::NodeDirection;
    /// 
    /// // Create parent and child nodes
    /// let mut parent_node = Node::<i32>::get_empty_node();
    /// let child_node = Node::<i32>::get_empty_node();
    /// 
    /// // Set the child node as the left child of the parent node
    /// parent_node.set_child_node(Some(child_node), 1.0, NodeDirection::LEFT);
    /// ```
    /// 
    /// # Notes
    /// 
    /// - If `direction` is `NodeDirection::LEFT`, the provided `node` becomes the left child of the current node.
    /// - If `direction` is `NodeDirection::RIGHT`, the provided `node` becomes the right child of the current node.
    /// - The `index` of the current node is updated to the specified value.
    /// - The provided `node` is wrapped in a `Box` before setting it as the child node to manage memory ownership.
    /// 
    /// This method is useful for constructing tree structures and organizing nodes based on their relationships.
    pub fn set_child_node(&mut self, node: Option<Self>, index: f32, direction: NodeDirection) {
        if direction == NodeDirection::LEFT && node.is_some() {
            self.left = Some(Box::new(node.unwrap()));
        }
        else if direction == NodeDirection::RIGHT && node.is_some() {
            self.right = Some(Box::new(node.unwrap()));
        }

        self.index = index;
    }

    /// Creates a leaf node for a tree with the given index and values.
    ///
    /// A leaf node represents a terminal node in a tree and holds a collection of values.
    ///
    /// # Arguments
    ///
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
    /// let leaf_node = Node::<i32>::create_leaf_node(vec![1, 2, 3]);
    /// ```
    ///
    /// # Notes
    ///
    /// - The `index` field is set to `0` to indicate that this node is a leaf.
    /// - The `is_leaf` field is set to `true` to indicate that this node is a leaf.
    /// - The `left` and `right` fields are set to `None` since leaf nodes do not have children.
    ///
    /// Leaf nodes are typically used in tree structures to represent endpoints or data storage points.
    pub fn create_leaf_node(values: Vec<T>) -> Self{
        Self {
            index: 0.0,
            values: Some(values),
            is_leaf: true,
            left: None,
            right: None
        }
    }
}