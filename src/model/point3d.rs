use std::cmp::Ordering;
use crate::functions::distance_calculator::DistanceCalculator;

#[derive(Debug)]
pub struct Point3D {
    x: f32,
    y: f32,
    z: f32
}

impl Point3D {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Point3D {x,y,z}
    }

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

    fn sorting_point(
        &self,
        other: &Point3D,
        axis: usize
    ) -> Ordering {

        // Compare x dimension
        if axis == 0 { self.x.partial_cmp(&other.x).unwrap() }

        // Compare y dimension
        else if axis == 1 { self.y.partial_cmp(&other.y).unwrap() }

        // Compare z dimension
        else { self.z.partial_cmp(&other.z).unwrap() }
    }
}

impl PartialEq for Point3D {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == self.z
    }
}

impl DistanceCalculator for Point3D {
    fn distance_to(&self, other: Self) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;

        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}
