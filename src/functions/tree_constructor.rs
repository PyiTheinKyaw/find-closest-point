pub trait TreeConstructor<T>
{
    fn get_constructor(points: Vec<T>, k: usize) -> (Option<Vec<T>>, Option<Vec<T>>, usize) ;

    fn spatial_partition_dataset(self) -> (Vec<T>, Vec<T>, usize);
}