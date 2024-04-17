use std::cell::RefCell;

pub struct SAH<T> {
    optimal_dimension: usize,
    optimal_split_value: f32,
}

impl<T> SAH<T> {
    fn select_optimal_splitting_plane(values: RefCell<Vec<T>>,  k: usize) -> (usize, usize){
        todo!()
    }
    fn calculate_sah_cost(sorted_list: &Vec<T>, k: usize, split_value: usize) -> f32 {
        todo!()
    }
    fn calculate_bounding_box(list: &Vec<T>) -> (T,T) {
        let min_coord = [f32::MAX, ]
    }
    fn partition_dataset(values: RefCell<Vec<T>>, split_value: usize) -> (&Vec<T>, &Vec<T>) {
        todo!()
    }
}