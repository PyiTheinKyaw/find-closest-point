use std::fmt::Debug;

use crate::model::direction::NodeDirection;
use crate::model::node::Node;
use crate::functions::tree_constructor::TreeConstructor;
use crate::functions::dataset::Dataset;
use crate::functions::sortable::Sortable;
use crate::model::sah::SAH;


#[derive(Debug)]
pub struct KDTree<T>
{
    pub root: Option<Node<T>>,
    dimension: usize
}

impl<T> KDTree<T>
where T: Dataset<T> + Sortable<T> + Debug
{
    /// Creates the kd-tree from the provided values.
    /// 
    /// This function constructs the KD-tree using the given values and parameters,
    /// such as the dimensionality of the tree (`k`) and the minimum number of points per subset.
    /// 
    /// # Arguments
    /// 
    /// * `values` - A vector containing the values to be used for constructing the tree.
    /// * `k` - The dimensionality of the KD-tree, representing the number of dimensions for each point.
    /// * `min_points_per_subset` - The minimum number of points allowed in each subset during tree construction.
    /// 
    /// # Returns
    /// 
    /// A result containing the constructed KD-tree on success, or an error message on failure.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use fnp::model::kd_tree::KDTree;
    /// use fnp::model::point3d::Point3D;
    ///
    /// // Create a KD-tree with root node from values, dimensionality 3, and minimum points per subset 1
    /// let points = vec![
    ///    Point3D::new(1.0, 2.0, 3.0),
    ///    Point3D::new(4.0, 52.0, 6.0),
    ///    Point3D::new(7.0, 8.0, 9.0),
    /// ];
    ///
    /// let kd_tree_result = KDTree::create_kd_tree(points, 3, 1);
    ///
    /// match kd_tree_result {
    ///     Ok(kd_tree) => {
    ///         // KD-tree creation succeeded, continue with further operations...
    ///         let root = kd_tree.root;
    ///
    ///     }
    ///     Err(error) => {
    ///         // KD-tree creation failed, handle the error...
    ///     }
    /// }
    /// ```
    /// 
    /// # Notes
    /// 
    /// - The root node of the KD-tree is constructed recursively using the `create_branch` function.
    /// - The dimensionality (`k`) of the KD-tree is set based on the provided value.
    /// - If successful, the function returns a `Result` containing the constructed KD-tree.
    /// - If an error occurs during tree construction, an error message is returned as part of the `Result`.
    /// 
    /// This function serves as the entry point for creating a KD-tree structure from a set of input values.
    pub fn create_kd_tree(values: Vec<T>, k: usize, min_points_per_subset: usize) -> Result<KDTree<T>, String> {
        let kd_tree = Self {
            root: Self::create_branch(values, 0, k, min_points_per_subset),
            dimension: k
        };
        
        Ok(kd_tree)
    }

    /// Creates a branch node of the KD-tree from the provided values.
    ///
    /// This function recursively constructs a branch node of the KD-tree using the given values,
    /// depth, dimensionality (`k`), and minimum number of points per subset.
    ///
    /// # Arguments
    ///
    /// * `values` - A vector containing the values to be used for constructing the branch node.
    /// * `depth` - The current depth level of the branch node within the tree structure.
    /// * `k` - The dimensionality of the KD-tree, representing the number of dimensions for each point.
    /// * `min_points_per_subset` - The minimum number of points allowed in each subset during tree construction.
    ///
    /// # Returns
    ///
    /// A branch node of the KD-tree constructed from the provided values and parameters.
    /// # Notes
    ///
    /// - The branch node is constructed recursively, splitting the values into left and right subsets based on the current depth.
    /// - The depth parameter is used to determine the current dimension (axis) for splitting the values.
    /// - If the number of values falls below the specified minimum threshold, a leaf node is created instead.
    /// - The resulting branch node contains references to its left and right child nodes, which may further branch out in the tree.
    ///
    /// This function is a key component in the construction of the KD-tree, organizing points into hierarchical structures for efficient querying.
    fn create_branch
    (
        values: Vec<T>,
        depth: usize,
        k: usize,
        min_points_per_subset: usize
    ) -> Option<Node<T>> {
        if values.len() <= min_points_per_subset {
            return Some(Node::create_leaf_node(values));
        }

        // Determine current dimension(axis) based on depth (alternates between x, y, z)
        let axis = depth % k;

        let (left_subset, right_subset, index, sah_cost) = SAH::get_constructor(values, axis);

        return if sah_cost != 0.0 {
            let mut node = Node::get_empty_node();

            if left_subset.is_some(){
                node.set_child_node(
                    Self::create_branch(left_subset.unwrap(), depth + 1, k, min_points_per_subset),
                    index,
                    NodeDirection::LEFT
                );
            }

            if right_subset.is_some() {
                node.set_child_node(
                    Self::create_branch(right_subset.unwrap(), depth + 1, k, min_points_per_subset),
                    index,
                    NodeDirection::RIGHT
                );
            }

            Some(node)
        } else {
            let leave_node = if left_subset.is_some() {
                Some(Node::create_leaf_node(left_subset.unwrap()))
            }
            else if right_subset.is_some(){
                Some(Node::create_leaf_node(right_subset.unwrap()))
            }
            else {None};
            leave_node
        }
    }
}
