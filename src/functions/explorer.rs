pub trait Explorer<T> {
    type SearchOutput;
    fn nearest_neighbour(&self, max_distance_sq: f32, query_point: &T) -> Box<(f32, &Self::SearchOutput)>;
    fn k_nearest_neighbour(&self, max_distance_sq: f32, query_point: &T) -> Box<Vec<(f32, &Self::SearchOutput)>>;
}