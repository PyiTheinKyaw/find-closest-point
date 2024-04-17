pub trait Dataset<T> {
    fn generate_data_list(amount: usize, min: f32, max: f32) -> Vec<T>;
    fn random_data(min: f32, max: f32) -> T;
}