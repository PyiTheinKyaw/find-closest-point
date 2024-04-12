use std::cmp::Ordering;
use std::rc::Rc;

use crate::tree::error_handler::ComparisonError;
#[derive(Debug, PartialEq)]
pub enum NodeDirection {
    LEFT,
    RIGHT
}

/*
Interface KDTree
=================
The Collection interface declares one or multiple methods for
getting iterators compatible with the collection. Note that the
return type of the methods must be declared as the iterator
interface so that the concrete collections can return various
kinds of iterators.
*/
pub trait IKDTree<P>
{
    type Output;

    /**
     @param
        point: is lists of point object.
        depth: is used to calculate the axis which is used to compare dimension .
        k: is the dimension .
     **/
    fn create_kd_tree(points: &mut Vec<P>, depth: usize, k: usize) -> Result<Rc<Self::Output>, String>;

    fn build_kd_tree
    (
        init_kd_tree: Self::Output,
        points: &mut Vec<P>,
        k: usize,
        depth: usize,
    ) -> Option<Rc<Self::Output>>;

    // .........
    fn init() -> Self::Output;

    fn sorting_point(
        point_a: &P,
        point_b: &P,
        axis: usize
    ) -> Result<Ordering, ComparisonError>;

    fn sorting_nearest(
        n_point_a: &(f32, &P),
        n_point_b: &(f32, &P),
    ) -> Result<Ordering, ComparisonError>;

    fn operation_point_list(
        points: &Vec<P>,
        median: usize,
        direction: NodeDirection
    ) -> &[P];

    fn find_closest(
        &self,
        query_point: &P,
        k: usize,
        point_limit: usize
    ) -> Option<Vec<(f32, &P)>>;

    fn nearest_neighbour
    <'p>
    (
        node: &'p Self::Output,
        max_distance_sq: f32,
        query_point: &P,
        best_points: Vec<(f32, &'p P)>,
        k: usize
    ) -> Vec<(f32, &'p P)>;

    fn direction(query_point: &P, node_point: &P, axis: usize) -> NodeDirection;
}

/*Interface Iterator */
pub trait IIterator {
    fn get_next();
    fn has_more() -> bool;
}