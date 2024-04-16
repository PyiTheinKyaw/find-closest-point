
pub trait Tree<T> {

    type Output;

    fn create_tree(values: &mut Vec<T>, depth: usize, k: usize) -> Result<Self::Output, String>;
    fn find_closest(&self, query_point: &T) ->Box<(f32, &T)>;
    fn find_k_closest(&self, query_point: &T, limit: usize) -> Box<Vec<(f32, &T)>>;
}