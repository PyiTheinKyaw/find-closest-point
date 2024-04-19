use std::cell::RefCell;
use std::fmt::Debug;

use crate::model::node::Node;
use crate::functions::tree_constructor::TreeConstructor;
use crate::functions::dataset::Dataset;
use crate::model::point3d::Point3D;

#[derive(Debug)]
pub struct KDTree<T>
{
    root: Node<T>,
    dimension: usize
}

impl KDTree<Point3D> {
    fn new(root: Node<Point3D>, dimension: usize) -> Self {
        Self {root, dimension}
    }

    fn create_tree(
        values: RefCell<Vec<Point3D>>,
        depth: usize,
        k: usize,
        constructor: Box<impl TreeConstructor<Point3D>>
    ) -> Result<KDTree<Point3D>, String> {
        todo!()
    }

    /// Partitions a dataset of 3D points into two subsets based on a split value along a specified dimension.
    ///
    /// # Arguments
    ///
    /// * `values` - A vector containing the 3D points to be partitioned.
    /// * `split_value` - The value used to split the dataset along the specified dimension.
    /// * `dimension` - The dimension (0 for x, 1 for y, 2 for z) along which the dataset is split.
    ///
    /// # Returns
    ///
    /// A tuple containing two vectors: the left subset of points where the coordinate value along the specified dimension is less than the split value,
    /// and the right subset of points where the coordinate value along the specified dimension is greater than or equal to the split value.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::fnp::model::point3d::Point3D;
    /// use crate::fnp::model::kd_tree::KDTree;
    /// let points = vec![
    ///     Point3D { x: 1.0, y: 2.0, z: 3.0 },
    ///     Point3D { x: 2.0, y: 3.0, z: 4.0 },
    ///     Point3D { x: 3.0, y: 4.0, z: 5.0 },
    /// ];
    /// let (left, right) = KDTree::partition_dataset(points, 2.5, 0);
    /// assert_eq!(left.len(), 2);
    /// assert_eq!(right.len(), 1);
    /// ```
    ///
    /// This function iterates over each point in the dataset and categorizes it into either the left or right subset based on its coordinate value along the specified dimension.

    pub fn partition_dataset(
        values: Vec<Point3D>,
        split_value: f32,
        dimension: usize
    ) -> (Vec<Point3D>, Vec<Point3D>) {

        let mut left_subset: Vec<Point3D> = vec![];
        let mut right_subset: Vec<Point3D> = vec![];

        for point in values.into_iter() {

            let mut value: &f32;

            if dimension == 0 { value = &point.x }
            else if dimension == 1 { value = &point.y }
            else { value = &point.z }


            // Check the coordinate value along the specified dimension
            if value < &split_value {
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
        let (left, right) = KDTree::partition_dataset(points.clone(), 2.5, 0);

        // Ensure correct partitioning
        assert_eq!(left.len(), 2);
        assert_eq!(right.len(), 2);
        assert!(left.contains(&Point3D { x: 1.0, y: 2.0, z: 3.0 }));
        assert!(left.contains(&Point3D { x: 2.0, y: 3.0, z: 4.0 }));
        assert!(right.contains(&Point3D { x: 3.0, y: 4.0, z: 5.0 }));
        assert!(right.contains(&Point3D { x: 4.0, y: 5.0, z: 6.0 }));

        // Split the dataset based on y dimension with split value of 3.5
        let (left, right) = KDTree::partition_dataset(points.clone(), 3.5, 1);

        // Ensure correct partitioning
        assert_eq!(left.len(), 2);
        assert_eq!(right.len(), 2);
        assert!(left.contains(&Point3D { x: 1.0, y: 2.0, z: 3.0 }));
        assert!(left.contains(&Point3D { x: 2.0, y: 3.0, z: 4.0 }));
        assert!(right.contains(&Point3D { x: 3.0, y: 4.0, z: 5.0 }));
        assert!(right.contains(&Point3D { x: 4.0, y: 5.0, z: 6.0 }));

        // Split the dataset based on z dimension with split value of 4.5
        let (left, right) = KDTree::partition_dataset(points.clone(), 4.5, 2);

        // Ensure correct partitioning
        assert_eq!(left.len(), 2);
        assert_eq!(right.len(), 2);
        assert!(left.contains(&Point3D { x: 1.0, y: 2.0, z: 3.0 }));
        assert!(left.contains(&Point3D { x: 2.0, y: 3.0, z: 4.0 }));
        assert!(right.contains(&Point3D { x: 3.0, y: 4.0, z: 5.0 }));
        assert!(right.contains(&Point3D { x: 4.0, y: 5.0, z: 6.0 }));
    }
}

