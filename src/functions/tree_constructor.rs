use std::cell::RefCell;
use crate::functions::sortable::Sortable;

pub trait TreeConstructor<T>: Sortable<T> {
    fn get_subset(values: RefCell<Vec<T>>) -> (Vec<T>, Vec<T>);
}