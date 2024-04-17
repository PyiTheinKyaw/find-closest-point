use std::cmp::Ordering;
use crate::functions::dataset::Dataset;

pub trait Sortable<T>: Dataset<T>
{
    fn sort_with_axis(&self, other: &T, axis: usize) -> Ordering;
}