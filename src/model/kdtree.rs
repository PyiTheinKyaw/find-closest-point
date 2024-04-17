use std::cell::RefCell;
use std::fmt::Debug;

use crate::model::node::Node;
use crate::functions::tree_constructor::TreeConstructor;
use crate::model::point3d::Point3D;

#[derive(Debug)]
pub struct KDTree<T>
{
    root: Node<T>,
    dimension: usize
}

impl KDTree<Point3D> {
    fn new(root: Node<Point3D>, dimension: usize) -> Self {
        Self {root, dimension}
    }

    fn create_tree(
        values: RefCell<Vec<Point3D>>,
        depth: usize,
        k: usize,
        constructor: Box<impl TreeConstructor<Point3D>>
    ) -> Result<KDTree<Point3D>, String> {
        todo!()
    }
}
