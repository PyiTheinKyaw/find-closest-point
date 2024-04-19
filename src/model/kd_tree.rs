use std::cell::RefCell;
use std::fmt::Debug;

use crate::model::node::Node;
use crate::functions::tree_constructor::TreeConstructor;
use crate::functions::dataset::Dataset;
use crate::model::point3d::Point3D;

#[derive(Debug)]
pub struct KDTree<T>
{
    root: Node<T>,
    dimension: usize
}

impl<T> KDTree<T>
where T: Dataset<T>
{
    fn new(root: Node<T>, dimension: usize) -> Self {
        Self {root, dimension}
    }

    fn create_tree(
        values: RefCell<Vec<T>>,
        depth: usize,
        k: usize,
        constructor: Box<impl TreeConstructor<T>>
    ) -> Result<KDTree<T>, String> {
        todo!()
    }
}
