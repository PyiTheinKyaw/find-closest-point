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
    root: Node<T>,
    dimension: usize
}

impl<T> KDTree<T>
where T: Dataset<T> + Sortable<T> + TreeConstructor<T> + Debug
{
    /// Creates the root node of a KD-tree from the provided values.
    /// 
    /// This function constructs the root node of a KD-tree using the given values and parameters,
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
    /// use your_module::KDTree;
    /// 
    /// // Create a KD-tree with root node from values, dimensionality 3, and minimum points per subset 5
    /// let kd_tree_result = KDTree::<i32>::create_root(values, 3, 5);
    /// 
    /// match kd_tree_result {
    ///     Ok(kd_tree) => {
    ///         // KD-tree creation succeeded, continue with further operations...
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
    fn create_root(values: Vec<T>, k: usize, min_points_per_subset: usize) -> Result<KDTree<T>, String> {
        let kd_tree = Self {
            root: Self::create_branch(values, 0, k, min_points_per_subset),
            dimension: k
        };
        
        Ok(kd_tree)
    }

    fn create_branch
    (
        values: Vec<T>,
        depth: usize,
        k: usize,
        min_points_per_subset: usize
    ) -> Node<T>{
        if values.len() <= min_points_per_subset {
            return Node::create_leaf_node(values);
        }

        // Determine current dimension(axis) based on depth (alternates between x, y, z)
        let axis = depth % k;

        let (left_subset, right_subset, index) = SAH::get_constructor(values, axis);

        let mut node = Node::get_empty_node();

        if left_subset.is_some() {
            node.set_child_node(
                Self::create_branch(left_subset.unwrap(), depth+1, k, min_points_per_subset), 
                index, 
                NodeDirection::LEFT
            );
        }
        
        if right_subset.is_some() {
            node.set_child_node(
                Self::create_branch(right_subset.unwrap(), depth+1, k, min_points_per_subset), 
                index,
                NodeDirection::RIGHT
            );
        }
        
        node
    }
}
