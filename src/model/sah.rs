use std::cell::RefCell;
use std::rc::Rc;
use crate::functions::dataset::Dataset;
use crate::functions::sortable::Sortable;
use crate::model::bounding_box::BoundingBox;
use crate::model::kd_tree::KDTree;
use crate::model::point3d::Point3D;

pub struct SAH {
    optimal_dimension: usize,
    optimal_split_value: f32,
}

impl SAH
{
    fn select_optimal_splitting_plane<T>(values: RefCell<Vec<T>>,  k: usize) -> (usize, usize)
    where T: Sortable<T>
    {
        let min_cost = f32::MAX;
        let optimal_dimension: f32 = 0.0;
        let optimal_split_value: f32 = 0.0;
        let mut point_list = values.borrow_mut();

        // Iterate over all dimensions
        for axis in 0..k {
            // Sort points along the current dimension
            let sorted_points = point_list.sort_by(|a, b| a.sort_with_axis(&b, axis));

        }

        todo!()
    }

    /// Finds the dimension (axis) with the largest range of coordinate values among a collection of points.
    ///
    /// This function iterates over each dimension (axis) of the points and calculates the range of coordinate
    /// values along that dimension. It then determines the dimension with the largest range and returns its index.
    ///
    /// # Arguments
    ///
    /// * `points` - A reference to a vector of points, where each point contains coordinates in multiple dimensions.
    /// * `k` - The dimensionality of the points (i.e., the number of dimensions).
    ///
    /// # Type Parameters
    ///
    /// * `T` - The type of the points. Must implement the `Dataset<T>` trait, providing access to the internal state.
    ///
    /// # Returns
    ///
    /// The index of the dimension (axis) with the largest range of coordinate values.
    ///
    /// # Example
    ///
    /// ```
    /// // Define a vector of points for testing
    /// use fnp::model::point3d::Point3D;
    /// use fnp::model::sah::SAH;
    /// 
    /// let points = vec![
    ///     Point3D::new(1.0, 2.0, 3.0),
    ///     Point3D::new(4.0, 5.0, 6.0),
    ///     Point3D::new(7.0, 8.0, 9.0),
    /// ];
    ///
    /// // Find the dimension with the largest range of coordinate values
    /// let largest_range_axis = SAH::find_dimension_axis_with_largest_range(&points, 3);
    /// assert_eq!(largest_range_axis, 0);
    /// ```
    ///
    /// This example demonstrates how to use the function to find the dimension with the largest range of coordinate
    /// values among a collection of points. In this case, the expected result is `2`, indicating the third dimension.
    ///
    /// # Note
    ///
    /// The function calculates the range of coordinate values along each dimension by iterating over all points and
    /// finding the minimum and maximum coordinate values for each dimension. It then compares these ranges to determine
    /// the dimension with the largest range. If multiple dimensions have the same largest range, the function returns
    /// the index of the first dimension encountered during iteration.
    /// 
    /// @author: Pyi Thein Kyaw
    /// 
    pub fn find_dimension_axis_with_largest_range<T>(points: &Vec<T>, k: usize) -> usize 
    where T: Dataset<T>
    {
        // Initialize variables to track dimension with largest range and its associated range value
        let mut largest_range_axis = 0;
        let mut largest_range_value = f32::MIN;

        // Iterate over each dimension
        for axis in 0..k {
            // Initialize variables to track minimum and maximum coordinate values along current dimension
            let mut min_coord = f32::MAX;
            let mut max_coord = f32::MIN;

            // Find minimum and maximum coordinate values along current dimension
            for point in points {
                let point_coord = point.get_internal_state()[axis];

                min_coord = point_coord.min(min_coord);
                max_coord = point_coord.max(max_coord);
            }

            // Calculate range of coordinate values along current dimension
            let range_value = max_coord - min_coord;

            // Update largest range dimension if current range is larger
            if range_value > largest_range_value {
                largest_range_axis = axis;
                largest_range_value = range_value;
            }
        }
        
        largest_range_axis
    }

    /// Calculates the Surface Area Heuristic (SAH) cost for splitting a dataset along a given axis.
    ///
    /// This function computes the SAH cost for splitting a dataset represented by `sorted_list` along
    /// the specified `axis` at the given `split_value`. The SAH cost is calculated as twice the sum
    /// of the surface areas of the bounding boxes of the two subsets resulting from the split.
    ///
    /// # Arguments
    ///
    /// * `sorted_list` - A ref of sorted list of elements representing the dataset.
    /// * `axis` - The axis along which to split the dataset (e.g., 0 for X-axis, 1 for Y-axis).
    /// * `k` - The dimension of the elements in the dataset.
    /// * `split_value` - The value used to split the dataset along the specified axis.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The type of the elements in the dataset. Must implement the `Dataset<T>` trait.
    ///
    /// # Returns
    ///
    /// The SAH cost for the split as a `f32` value.
    ///
    /// # Note
    ///
    /// This function first partitions the dataset into two subsets based on the `split_value` and `axis`.
    /// It then calculates the bounding boxes for the left and right subsets using the `calculate_bounding_box`
    /// method of the `BoundingBox` struct. Next, it computes the surface areas of the bounding boxes using
    /// the `calculate_surface_area` method. Finally, it returns twice the sum of the surface areas of the
    /// bounding boxes of the left and right subsets as the SAH cost for the split.
    ///
    /// # Example
    ///
    /// ```
    /// use fnp::model::point3d::Point3D;
    /// use fnp::model::sah::SAH;;
    /// 
    /// let sorted_list = vec![
    ///     Point3D::new(1.0, 2.0, 3.0),
    ///     Point3D::new(4.0, 5.0, 6.0),
    ///     Point3D::new(7.0, 8.0, 9.0),
    /// ];
    ///
    /// // Calculate the SAH cost for splitting along the X-axis at split value 4.0
    /// let sah_cost = SAH::calculate_sah_cost(&sorted_list, 0, 3, 4.0);
    /// ```
    ///
    /// This example calculates the SAH cost for splitting a dataset of 3D points along the X-axis at a split value of 4.0.
    /// The expected SAH cost will depend on the specific dataset and splitting criteria.
    /// 
    /// @author: Pyi Thein Kyaw
    pub fn calculate_sah_cost<T>(
        sorted_list: &Vec<T>,
        axis: usize,
        k: usize,
        split_value: f32
    ) -> f32 
    where T: Dataset<T>
    {
        // Partition dataset into two subsets based on split_value and axis of each dimension (x,y,z, etc..)
        let (left_subset, right_subset): (Vec<&T>, Vec<&T>) = Self::partition_dataset(sorted_list, split_value, axis);
        let (left_size, right_size) = (left_subset.len(), right_subset.len());
        
        let left_bounding_box = BoundingBox::calculate_bounding_box(left_subset, k);
        let right_bounding_box = BoundingBox::calculate_bounding_box(right_subset, k);

        let surface_area_left = left_bounding_box.calculate_surface_area();
        let surface_area_right = right_bounding_box.calculate_surface_area();

        2.0 * ((left_size as f32 * surface_area_left) + (right_size as f32 * surface_area_right))
    }

    /// Partitions a dataset into two subsets based on a split value along a specified axis.
    ///
    /// This function takes a dataset represented as a vector `values`, along with a `split_value`
    /// and an `axis` along which to perform the partitioning which is also axis of given dimension.
    ///
    /// It returns two vectors: `left_subset`
    /// containing the points whose coordinate value along the specified axis is less than the `split_value`,
    /// and `right_subset` containing the remaining points.
    ///
    /// # Arguments
    ///
    /// * `values` - A ref of vector containing the dataset to be partitioned.
    /// * `split_value` - The value used to partition the dataset along the specified axis.
    /// * `axis` - The index of the axis along which to perform the partitioning.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The type of the elements in the dataset. Must implement the `Dataset<T>` trait.
    ///
    /// # Returns
    ///
    /// A tuple `(left_subset, right_subset)` containing the left and right subsets of the dataset by ref
    /// after partitioning.
    ///
    /// # Note
    ///
    /// This function iterates over each point in the dataset and compares the value of the coordinate
    /// along the specified axis with the `split_value`. Points with coordinate values less than
    /// `split_value` are placed in the `left_subset`, while the rest are placed in the `right_subset`.
    ///
    /// # Examples
    ///
    /// ```
    /// use fnp::model::point3d::Point3D;
    /// use fnp::model::sah::SAH;
    ///
    /// let points = vec![Point3D::new(1.0, 2.0, 3.0), Point3D::new(4.0, 5.0, 6.0)];
    /// let (left_subset, right_subset) = SAH::partition_dataset(&points, 2.5, 0);
    /// ```
    ///
    /// This example partitions a dataset of 3D points along the X-axis with a split value of `2.5`.
    /// Points with X-coordinate less than `2.5` are placed in the left subset, while the rest are
    /// placed in the right subset.
    ///
    ///
    /// @author: Pyi Thein Kyaw
    pub fn partition_dataset<T>(
        values: &Vec<T>,
        median: f32,
        axis: usize
    ) -> (Vec<&T>, Vec<&T>)
    where T: Dataset<T>
    {

        let mut left_subset: Vec<&T> = vec![];
        let mut right_subset: Vec<&T> = vec![];

        for point in values.iter() {

            let point_coord = point.get_internal_state();

            let value = &point_coord[axis];

            // Check the coordinate value along the specified dimension
            if value < &median {
                left_subset.push(point);
            }
            // Point belongs to the right subset
            else {
                right_subset.push(point);
            }
        }

        (left_subset, right_subset)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partition_dataset() {
        // Define test points
        let points = vec![
            Point3D { x: 1.0, y: 2.0, z: 3.0 },
            Point3D { x: 2.0, y: 3.0, z: 4.0 },
            Point3D { x: 3.0, y: 4.0, z: 5.0 },
            Point3D { x: 4.0, y: 5.0, z: 6.0 },
        ];

        // Split the dataset based on x dimension with split value of 2.5
        let (left, right) = SAH::partition_dataset(&points, 2.5, 0);

        // Ensure correct partitioning
        assert_eq!(left.len(), 2);
        assert_eq!(right.len(), 2);
        assert!(left.contains(&&Point3D { x: 1.0, y: 2.0, z: 3.0 }));
        assert!(left.contains(&&Point3D { x: 2.0, y: 3.0, z: 4.0 }));
        assert!(right.contains(&&Point3D { x: 3.0, y: 4.0, z: 5.0 }));
        assert!(right.contains(&&Point3D { x: 4.0, y: 5.0, z: 6.0 }));

        // Split the dataset based on y dimension with split value of 3.5
        let (left, right) = SAH::partition_dataset(&points, 3.5, 1);

        // Ensure correct partitioning
        assert_eq!(left.len(), 2);
        assert_eq!(right.len(), 2);
        assert!(left.contains(&&Point3D { x: 1.0, y: 2.0, z: 3.0 }));
        assert!(left.contains(&&Point3D { x: 2.0, y: 3.0, z: 4.0 }));
        assert!(right.contains(&&Point3D { x: 3.0, y: 4.0, z: 5.0 }));
        assert!(right.contains(&&Point3D { x: 4.0, y: 5.0, z: 6.0 }));

        // Split the dataset based on z dimension with split value of 4.5
        let (left, right) = SAH::partition_dataset(&points, 4.5, 2);

        // Ensure correct partitioning
        assert_eq!(left.len(), 2);
        assert_eq!(right.len(), 2);
        assert!(left.contains(&&Point3D { x: 1.0, y: 2.0, z: 3.0 }));
        assert!(left.contains(&&Point3D { x: 2.0, y: 3.0, z: 4.0 }));
        assert!(right.contains(&&Point3D { x: 3.0, y: 4.0, z: 5.0 }));
        assert!(right.contains(&&Point3D { x: 4.0, y: 5.0, z: 6.0 }));
    }

    #[test]
    fn test_calculate_sah_cost() {
        // Create a sorted list of points for testing
        let sorted_list = vec![
            Point3D::new(1.0, 2.0, 3.0),
            Point3D::new(4.0, 5.0, 6.0),
            Point3D::new(7.0, 8.0, 9.0),
        ];

        // Calculate the SAH cost for splitting along the X-axis at split value 4.0
        let sah_cost = SAH::calculate_sah_cost(&sorted_list, 0, 3, 4.0);

        // Assert that the calculated SAH cost matches the expected value
        // The expected value can be calculated based on the surface areas of the bounding boxes
        assert_eq!(sah_cost, 216.0); 
    }

    #[test]
    fn test_find_dimension_axis_with_largest_range() {
        // Create a vector of points for testing
        let points = vec![
            Point3D::new(1.0, 2.0, 3.0),
            Point3D::new(4.0, 5.0, 6.0),
            Point3D::new(7.0, 8.0, 9.0),
        ];

        // Call the function to find the dimension with the largest range
        let largest_range_axis = SAH::find_dimension_axis_with_largest_range(&points, 3);

        // Assert that the result is as expected
        assert_eq!(largest_range_axis, 0);
    }
}


