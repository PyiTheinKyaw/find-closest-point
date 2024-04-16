use std::cell::RefCell;
use std::fmt::Debug;

use crate::model::node::Node;
use crate::functions::tree::Tree;
use crate::functions::distance_calculator::DistanceCalculator;

#[derive(Debug)]
pub struct KDTree<T>
{
    root: Node<T>,
    dimension: usize
}

impl<T> KDTree<T> {
    fn new(root: Node<T>, dimension: usize) -> Self {
        Self {root, dimension}
    }
}

impl<T> Tree<T> for KDTree<T>
where T: Debug + PartialEq + DistanceCalculator
{
    type Output = Self;

    fn create_tree(values: &mut Vec<T>, depth: usize, k: usize) -> Result<Self::Output, String> {
        // Implement Median approach partition here.
        let axis = depth % k;

        let median = values.len() / 2;
        let node = Node::get_empty_node(values.remove(median), depth + 1);

        // Median 0 means there is no points left to operate.
        if median != 0 {

            // Calculate the direction
            // If Median is 1 and len is 2, only left node to create.
            if median == 1 && values.len() == 2 {

            }
        }
        todo!()
    }

    fn find_closest(&self, query_point: &T) -> Box<(f32, &T)> {
        todo!()
    }

    fn find_k_closest(&self, query_point: &T, limit: usize) -> Box<Vec<(f32, &T)>> {
        todo!()
    }
}