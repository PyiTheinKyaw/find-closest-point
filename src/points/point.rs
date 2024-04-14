pub trait Point<T> {

    fn generate_points(amount: usize, min: f32, max: f32) -> Vec<T>;
    fn distance_to(&self, other: &Self) -> f32;
    fn random_point(min: f32, max: f32) -> T;

    fn get_coordinate(&self) -> Vec<&f32>;
}

#[derive(Debug, Copy, Clone)]
pub struct Point3D {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Point3D {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Point3D{x, y, z}
    }
}


impl Point<Point3D> for Point3D
{

    /*
    This method is used to generate points depends on amount parameter.
    */
    fn generate_points(amount: usize, min: f32, max: f32) -> Vec<Point3D> {
        let mut points = Vec::with_capacity(amount);
        for _ in 0..amount {
            points.push(Point3D::random_point(min, max));
        }
        points
    }

    /* 
    This method is used to calculate the distance between points.
    In order to get the distance between points, used Euclidean Distance 
    */
    fn distance_to(&self, other: &Self) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;

        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    /* This method is used to generate random one points. */
    fn random_point(min: f32, max: f32) -> Self {
        Point3D::new(
            ((rand::random::<f32>() * (max - min) + min)* 100.0).round() / 100.0,
            ((rand::random::<f32>() * (max - min) + min)* 100.0).round() / 100.0,
            ((rand::random::<f32>() * (max - min) + min)* 100.0).round() / 100.0,
        )
    }

    fn get_coordinate(&self) -> Vec<&f32> {
        vec![&self.x, &self.y, &self.z]
    }
}

impl PartialEq for Point3D {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == self.z
    }
}


#[cfg(test)]
mod tests {
    use crate::points::point::{Point3D, Point};
    #[test]
    fn test_single_point() {
        let p = Point3D::new(1.2, 3.4, 5.6);
        assert_eq!(p.x, 1.2);
        assert_eq!(p.y, 3.4);
        assert_eq!(p.z, 5.6);
    }


    #[test]
    fn test_generate_points() {
        let amount = 5;
        let points = Point3D::generate_points(amount);
        assert_eq!(points.len(), amount);
        for point in &points {
            assert!(point.x.is_finite());
            assert!(point.y.is_finite());
            assert!(point.z.is_finite());
        }
    }

    #[test]
    fn test_distance_to() {
        let p1 = Point3D::new(1.0, 2.0, 3.0);
        let p2 = Point3D::new(4.0, 5.0, 6.0);
        let distance = p1.distance_to(&p2);
        assert!((distance - 5.196152422706632).abs() < 1e-8);
    }
}