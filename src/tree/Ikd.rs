use std::cell::{Ref, RefCell, RefMut};
use std::cmp::Ordering;
use std::rc::Rc;
use crate::KDTree;

use crate::tree::error_handler::ComparisonError;
#[derive(Debug, PartialEq)]
pub enum NodeDirection {
    LEFT,
    RIGHT,
    NOWHERE
}

impl PartialEq<NodeDirection> for &NodeDirection {
    fn eq(&self, other: &NodeDirection) -> bool {
        self == other
    }
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

    fn new
    (
        point: P,
        depth: usize,
    ) -> Self::Output;

    fn set_child_node(&mut self, node: Self::Output, direction: &NodeDirection);

    /**
     @param
        point: is lists of point object.
        depth: is used to calculate the axis which is used to compare dimension .
        k: is the dimension .
     **/
    fn create_kd_tree
    (
        points: &mut RefCell<&[P]>,
        depth: usize,
        k: usize
    ) -> Result<Box<Self::Output>, String>;

    // This is the helper function to do create_kd_tree.
    fn build_kd_tree
    (
        points: &mut RefCell<&[P]>,
        k: usize,
        depth: usize,
    ) -> Self::Output;

    // .........
    fn multi_dimensional_sort<'a>(list: &'a mut RefCell<&'a [P]>, axis: usize) -> &'a mut RefCell<&'a [P]>;

    fn sorting_nearest(
        n_point_a: &(f32, &P),
        n_point_b: &(f32, &P),
    ) -> Result<Ordering, ComparisonError>;

    fn operation_point_list
    <'kdp>
    (
        points: Ref<&'kdp [P]>,
        median: usize,
        direction: &NodeDirection
    ) -> RefCell<&'kdp [P]>;

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