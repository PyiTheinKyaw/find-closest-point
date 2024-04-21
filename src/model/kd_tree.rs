use std::fmt::Debug;

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
    fn new(root: Node<T>, dimension: usize) -> Self {
        Self {root, dimension}
    }

    fn create_branch
    (
        values: Vec<T>,
        depth: usize,
        k: usize
    ) -> Result<KDTree<T>, String> {
        // Determine current dimension(axis) based on depth (alternates between x, y, z)
        let axis = depth % k;

        let (left_subset, right_subset) = SAH::get_constructor(values, axis);
        let node = Node::create_internal_node(

        )


        todo!()
    }
}
