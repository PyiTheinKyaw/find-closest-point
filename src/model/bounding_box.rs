use crate::functions::dataset::Dataset;

pub struct BoundingBox {
    pub k: usize,
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
    /// * `list` - A vector containing the ref of points.
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
    /// 
    /// let pa = Point3D::new(1.0, 2.0, 3.0);
    /// let pb = Point3D::new(4.0, 5.0, 6.0);
    ///
    /// let points = vec![&pa, &pb];
    /// let bounding_box = BoundingBox::calculate_bounding_box(points, 3);
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
    ///
    /// @author: Pyi Thein Kyaw
    pub fn calculate_bounding_box<T>(list: Vec<&T>, k: usize) -> Self
    where T: Dataset<T>
    {
        let mut min_coordinates: Vec<f32> = Vec::with_capacity(k);
        let mut max_coordinates: Vec<f32> = Vec::with_capacity(k);

        // allocate on the memory according to dimension-axis value.
        unsafe {
            min_coordinates.set_len(k);
            max_coordinates.set_len(k);
        }

        // Init value to assert the max-min coord.
        for index in 0..k {
            min_coordinates[index] = f32::MAX;
            max_coordinates[index] = f32::MIN;
        }

        // Iterate over each point to update minimum and maximum coordinates
        for point in list.iter() {

            // For each axis in dimension.
            for index in 0..k {

                let point_coord = point.get_internal_state()[index];

                if min_coordinates[index] > point_coord { min_coordinates[index] = point_coord;}
                if max_coordinates[index] < point_coord { max_coordinates[index] = point_coord;}
            }
        }

        BoundingBox {k, min_coordinates, max_coordinates}
    }

    /// Calculates the surface area of the bounding box.
    ///
    /// This method calculates the surface area of the bounding box represented by the `BoundingBox` instance.
    /// It takes the dimension `k` as input, which specifies the number of dimensions of the bounding box.
    ///
    /// # Returns
    ///
    /// The surface area of the bounding box as a `f32`.
    ///
    /// # Examples
    ///
    /// ```
    /// use fnp::model::bounding_box::BoundingBox; 
    /// let bbox = BoundingBox {
    ///     k: 3,
    ///     min_coordinates: vec![0.0, 0.0, 0.0],
    ///     max_coordinates: vec![1.0, 2.0, 3.0],
    /// };
    /// let surface_area = bbox.calculate_surface_area();
    /// assert_eq!(surface_area, 22.0);
    /// ```
    ///
    /// This example creates a `BoundingBox` instance with dimensions 1x2x3 and calculates its surface area.
    /// The expected surface area for a cuboid with sides of length 1, 2, and 3 is 22.
    ///
    /// # Note
    ///
    /// This method calculates the surface area of the bounding box using the lengths of its sides along each axis.
    /// It computes the length of each side by subtracting the minimum coordinate from the maximum coordinate
    /// along each axis, then multiplies these lengths together and doubles the result to account for both
    /// the top and bottom faces of the bounding box.
    ///
    /// # Panics
    ///
    /// This method may panic if the provided dimension `k` is greater than the length of either `min_coordinates`
    /// or `max_coordinates`.
    /// 
    /// 
    /// @author: Pyi Thein Kyaw
    pub fn calculate_surface_area(&self) -> f32 {

        let mut axis_list = Vec::new();
        let mut surface_area: f32 = 0.0;

        for index in 0..self.k {
            axis_list.push(&self.max_coordinates[index] - &self.min_coordinates[index]);
        }

        for (index, elements) in axis_list.iter().enumerate() {
            
            let next_index = (index + 1)  % axis_list.len();
            surface_area += elements * axis_list[next_index];
        }

        surface_area = surface_area * 2.0;

        surface_area        
    }
}

#[cfg(test)]
mod tests {
    
    use crate::model::point3d::Point3D;
    use crate::model::bounding_box::BoundingBox;

    #[test]
    fn test_calculate_bounding_box() {
        // Create a list of points for testing
        let points = vec![
            &Point3D { x: 1.0, y: 2.0, z: 3.0 },
            &Point3D { x: 2.0, y: 3.0, z: 4.0 },
            &Point3D { x: 3.0, y: 4.0, z: 5.0 },
            &Point3D { x: 4.0, y: 5.0, z: 6.0 },
        ];

        // Call the function to calculate the bounding box
        let bounding_box = BoundingBox::calculate_bounding_box(points, 3);

        // Assert that the minimum and maximum coordinates are correct
        assert_eq!(bounding_box.min_coordinates, vec![1.0, 2.0, 3.0]);
        assert_eq!(bounding_box.max_coordinates, vec![4.0, 5.0, 6.0]);
    }

    #[test]
    fn test_calculate_surface_area() {
        // Create a BoundingBox instance for testing
        let bbox = BoundingBox {
            k: 3,
            min_coordinates: vec![0.0, 0.0, 0.0],
            max_coordinates: vec![1.0, 2.0, 3.0],
        };

        // Call the function to calculate the surface area
        let surface_area = bbox.calculate_surface_area();

        // Assert that the calculated surface area is correct
        // The expected surface area for a cuboid with sides of length 1, 2, and 3 is 22
        assert_eq!(surface_area, 22.0);
    }
}
