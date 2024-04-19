use std::cell::RefCell;
use std::rc::Rc;
use crate::functions::dataset::Dataset;
use crate::functions::sortable::Sortable;

pub struct SAH {
    optimal_dimension: usize,
    optimal_split_value: f32,
}

impl SAH
{
    fn select_optimal_splitting_plane<T>(values: RefCell<Vec<T>>,  k: usize) -> (usize, usize)
    where T: Sortable<T>
    {
        todo!()
    }
    fn calculate_sah_cost<T>(sorted_list: &Vec<T>, k: usize, split_value: usize) -> f32 {
        todo!()
    }
}

