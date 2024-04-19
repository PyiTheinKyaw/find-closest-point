use std::cell::RefCell;
use std::rc::Rc;
use crate::functions::dataset::Dataset;
use crate::functions::sortable::Sortable;
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
        todo!()
    }
    fn calculate_sah_cost<T>(sorted_list: &Vec<T>, k: usize, split_value: usize) -> f32 {
        todo!()
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
    /// * `values` - A vector containing the dataset to be partitioned.
    /// * `split_value` - The value used to partition the dataset along the specified axis.
    /// * `axis` - The index of the axis along which to perform the partitioning.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The type of the elements in the dataset. Must implement the `Dataset<T>` trait.
    ///
    /// # Returns
    ///
    /// A tuple `(left_subset, right_subset)` containing the left and right subsets of the dataset
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
    /// let (left_subset, right_subset) = SAH::partition_dataset(points, 2.5, 0);
    /// ```
    ///
    /// This example partitions a dataset of 3D points along the X-axis with a split value of `2.5`.
    /// Points with X-coordinate less than `2.5` are placed in the left subset, while the rest are
    /// placed in the right subset.
    ///
    ///
    /// @author: Pyi Thein Kyaw
    pub fn partition_dataset<T>(
        values: Vec<T>,
        split_value: f32,
        axis: usize
    ) -> (Vec<T>, Vec<T>)
    where T: Dataset<T>
    {

        let mut left_subset: Vec<T> = vec![];
        let mut right_subset: Vec<T> = vec![];

        for point in values.into_iter() {

            let mut value: &f32;
            let point_coord = point.get_internal_state();

            value = &point_coord[axis];

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
        let (left, right) = SAH::partition_dataset(points.clone(), 2.5, 0);

        // Ensure correct partitioning
        assert_eq!(left.len(), 2);
        assert_eq!(right.len(), 2);
        assert!(left.contains(&Point3D { x: 1.0, y: 2.0, z: 3.0 }));
        assert!(left.contains(&Point3D { x: 2.0, y: 3.0, z: 4.0 }));
        assert!(right.contains(&Point3D { x: 3.0, y: 4.0, z: 5.0 }));
        assert!(right.contains(&Point3D { x: 4.0, y: 5.0, z: 6.0 }));

        // Split the dataset based on y dimension with split value of 3.5
        let (left, right) = SAH::partition_dataset(points.clone(), 3.5, 1);

        // Ensure correct partitioning
        assert_eq!(left.len(), 2);
        assert_eq!(right.len(), 2);
        assert!(left.contains(&Point3D { x: 1.0, y: 2.0, z: 3.0 }));
        assert!(left.contains(&Point3D { x: 2.0, y: 3.0, z: 4.0 }));
        assert!(right.contains(&Point3D { x: 3.0, y: 4.0, z: 5.0 }));
        assert!(right.contains(&Point3D { x: 4.0, y: 5.0, z: 6.0 }));

        // Split the dataset based on z dimension with split value of 4.5
        let (left, right) = SAH::partition_dataset(points.clone(), 4.5, 2);

        // Ensure correct partitioning
        assert_eq!(left.len(), 2);
        assert_eq!(right.len(), 2);
        assert!(left.contains(&Point3D { x: 1.0, y: 2.0, z: 3.0 }));
        assert!(left.contains(&Point3D { x: 2.0, y: 3.0, z: 4.0 }));
        assert!(right.contains(&Point3D { x: 3.0, y: 4.0, z: 5.0 }));
        assert!(right.contains(&Point3D { x: 4.0, y: 5.0, z: 6.0 }));
    }
}


