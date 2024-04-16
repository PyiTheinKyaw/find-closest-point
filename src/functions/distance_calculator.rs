pub trait DistanceCalculator {
    fn distance_to(&self,  other: Self) -> f32;
}