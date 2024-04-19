use std::process::id;
use crate::functions::dataset::Dataset;
use crate::model::point3d::Point3D;

pub struct BoundingBox {
    k: usize,
    pub min_coordinates: Vec<f32>,
    pub max_coordinates: Vec<f32>,
}

impl BoundingBox {
    /// Calculates the bounding box for a list of points.
    ///
    /// This function calculates the bounding box for a list of points of type `T`.
    /// It takes the dimension `k` and a reference to the list of points as input parameters.
    ///
    /// # Arguments
    ///
    /// * `list` - A reference to a vector containing the points.
    /// * `k` - The dimension of the points (number of coordinates).
    ///
    /// # Type Parameters
    ///
    /// * `T` - The type of the points. Must implement the `Dataset<T>` trait.
    ///
    /// # Returns
    ///
    /// A `BoundingBox` instance representing the calculated bounding box.
    ///
    /// # Examples
    ///
    /// ```
    /// use fnp::model::point3d::Point3D;
    /// use fnp::model::bounding_box::BoundingBox;
    /// use super::*;
    ///
    /// let points = vec![Point3D::new(1.0, 2.0, 3.0), Point3D::new(4.0, 5.0, 6.0)];
    /// let bounding_box = BoundingBox::calculate_bounding_box(3, &points);
    /// ```
    ///
    /// # Panics
    ///
    /// This function may panic if any of the points does not have the expected dimension.
    ///
    /// # Note
    ///
    /// The minimum and maximum coordinates are calculated by iterating over each point and each axis in the dimension.
    /// The minimum coordinates are initialized with the maximum possible value of `f32`, and the maximum coordinates
    /// are initialized with the minimum possible value of `f32`, ensuring that any point will update the bounding box.
    /// If the list of points is empty, the resulting bounding box will have invalid coordinates.
    pub fn calculate_bounding_box<T>(list: &Vec<T>, k: usize) -> Self
    where T: Dataset<T>
    {
        let mut min_coordinates: Vec<f32> = Vec::new();
        let mut max_coordinates: Vec<f32> = Vec::new();

        // Iterate over each point to update minimum and maximum coordinates
        for point in list.iter() {

            // For each axis in dimension.
            for index in 0..k {

                let point_coord = point.get_internal_state()[index];

                min_coordinates.insert(index, point_coord.min(f32::MAX));
                max_coordinates.insert(index, point_coord.max(f32::MIN));
            }
        }

        BoundingBox {k, min_coordinates, max_coordinates}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_calculate_bounding_box() {
        // Create a list of points for testing
        let points = vec![
            Point3D { x: 1.0, y: 2.0, z: 3.0 },
            Point3D { x: 2.0, y: 3.0, z: 4.0 },
            Point3D { x: 3.0, y: 4.0, z: 5.0 },
            Point3D { x: 4.0, y: 5.0, z: 6.0 },
        ];

        // Call the function to calculate the bounding box
        let bounding_box = BoundingBox::calculate_bounding_box(3, &points);

        // Assert that the minimum and maximum coordinates are correct
        assert_eq!(bounding_box.min_coordinates, vec![1.0, 2.0, 3.0]);
        assert_eq!(bounding_box.max_coordinates, vec![4.0, 5.0, 6.0]);
    }
}
