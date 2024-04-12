use std::cmp::{Ordering, PartialEq};
use std::fmt::Debug;
use std::rc::Rc;
use crate::points::point::{Point, Point3D};
use crate::tree::error_handler::ComparisonError;
use super::Ikd::{IIterator, IKDTree, NodeDirection};

#[derive(Debug)]
pub struct KDTree<P>
{
    pub point: Option<Rc<P>>,
    depth: usize,
    pub left: Option<Rc<KDTree<P>>>,
    pub right: Option<Rc<KDTree<P>>>,
}

/**
Implementation of KDTree
**/
impl<P> IKDTree<P> for KDTree<P>
    where P: Point<Point3D> + Copy + PartialEq + Debug
{
    type Output = KDTree<P>;

    fn create_kd_tree(points: &mut Vec<P>, depth: usize, k: usize) -> Result<Rc<KDTree<P>>, String> {
        if points.len() == 0 {
            return Err(String::from("KDTreeBuildError: point len is zero."));
        }

        // In order to get almost perfect balance tree, we have to sort it first.
        points.sort_by(|a, b| Self::sorting_point(a, b, 0).unwrap());

        // Following code will init an KDTree object with zero value.
        if let Some(kd_tree) = Self::build_kd_tree(
            Self::init(),
            points,
            3,
            0
        ){
            return Ok(kd_tree)
        }else {
            Err(String::from("KDTreeBuildError: Error occurs while building KDTree"))
        }
    }

    fn build_kd_tree
    (
        mut init_kd_tree: Self::Output,
        sorted_points: &mut Vec<P>,
        k: usize,
        depth: usize
    ) -> Option<Rc<Self::Output>>
    {
        let axis = depth % k;

        // In order to get almost perfect balance tree, we have to sort it first.
        sorted_points.sort_by(|a, b| Self::sorting_point(a, b, axis).unwrap());

        // find the median
        let median = sorted_points.len() / 2;

        // Update current node.
        init_kd_tree.point = Some(Rc::new(sorted_points[median]));
        init_kd_tree.depth = depth;

        // Median 0 means there is no points left to operate.
        if median != 0 {

            // Only left node to create
            if median == 1 && sorted_points.len() == 2 {
                // Create Left nodes
                let left_point = Self::operation_point_list(
                    &sorted_points,
                    median,
                    NodeDirection::LEFT
                );

                match Self::build_kd_tree(
                    Self::init(),
                    &mut left_point.to_vec(),
                    3,
                    depth + 1
                )
                {
                    Some(left_child_node) => {
                        init_kd_tree.left = Some(Rc::clone(&left_child_node));
                    },

                    _ => ()
                }
            } else {
                // Create Left nodes
                let left_point = Self::operation_point_list(
                    &sorted_points,
                    median,
                    NodeDirection::LEFT
                );

                match Self::build_kd_tree(
                    Self::init(),
                    &mut left_point.to_vec(),
                    3,
                    depth + 1
                )
                {
                    Some(left_child_node) => {
                        init_kd_tree.left = Some(Rc::clone(&left_child_node));
                    },

                    _ => ()
                }

                // Create Right nodes
                let right_point = Self::operation_point_list(
                    &sorted_points,
                    median,
                    NodeDirection::RIGHT
                );

                match Self::build_kd_tree(
                    Self::init(),
                    &mut right_point.to_vec(),
                    3,
                    depth + 1
                )
                {
                    Some(right_child_node) => {
                        init_kd_tree.right = Some(Rc::clone(&right_child_node));
                    },

                    _ => ()
                }
            }
        }
                Some(Rc::new(init_kd_tree))

    }

    fn init() -> Self::Output {
        KDTree {
            point: None,
            depth: 0,
            left: None,
            right: None,
        }
    }

    fn sorting_point(
        point_a: &P,
        point_b: &P,
        axis: usize
    ) -> Result<Ordering, ComparisonError>
    {
        let point_a_cord = point_a.get_coordinate();
        let point_b_cord = point_b.get_coordinate();

        match axis {
            // Compare x dimension
            0 => Ok(point_a_cord[0].partial_cmp(&point_b_cord[0]).unwrap()),

            // Compare y dimension
            1 => Ok(point_a_cord[1].partial_cmp(&point_b_cord[1]).unwrap()),

            // Compare z dimension
            2 => Ok(point_a_cord[2].partial_cmp(&point_b_cord[2]).unwrap()),

            _ => Err(
                ComparisonError::InvalidOrdering(
                    "KDTreeBuildError: Sorting undone.".to_string()
                )
            )
        }
    }

    fn sorting_nearest(
        n_point_a: &(f32, &P),
        n_point_b: &(f32, &P),
    ) -> Result<Ordering, ComparisonError> {
        Ok(n_point_a.0.partial_cmp(&n_point_b.0).unwrap())
    }

    fn operation_point_list(
        points: &Vec<P>,
        median: usize,
        node_direction: NodeDirection
    ) -> &[P]
    {
        match node_direction {
            NodeDirection::LEFT => {
                &points[..median]
            }
            NodeDirection::RIGHT => {
                &points[median+1..]
            }
        }
    }

    fn find_closest(&self, query_point: &P, k: usize, point_limit: usize) -> Option<Vec<(f32, &P)>> {
        let mut best_points_list = vec![];
        best_points_list = Self::nearest_neighbour(
            &self,
            f32::MAX,
            query_point,
            best_points_list,
            k
        );

        best_points_list.sort_by(|a, b| Self::sorting_nearest(a,b).unwrap());

        if best_points_list.len() >= point_limit {
            return Some(best_points_list[..point_limit].to_vec());
        }

        else if best_points_list.len() > 0 {
           return Some(best_points_list);
        }

        None
    }

    fn nearest_neighbour
    <'p>
    (
        node: &'p Self::Output,
        mut max_distance_sq: f32,
        query_point: &P,
        mut best_points: Vec<(f32, &'p P)>,
        k: usize
    ) -> Vec<(f32, &'p P)>
    {
        let axis = node.depth % k;
        let point = node.point.as_ref().unwrap().as_ref();

        let left_node = node.left.as_ref();
        let right_node = node.right.as_ref();

        // Calculate the distance between current node and query point.
        let mut current_node_distance = query_point.distance_to(point);

        // Only current_dist is shorter than root dist.
        if current_node_distance < max_distance_sq {

            max_distance_sq = current_node_distance;
            best_points.push((max_distance_sq, point));

            // if it's not leaf then decided to choose left or right.
            let mut direction = Self::direction(query_point, point, axis);

            // Make sure Direction have node.
            if (direction == NodeDirection::LEFT && !left_node.is_none()) ||
                (direction == NodeDirection::RIGHT && !right_node.is_none())
            {
                let mut distance_to_op_side = f32::MAX;

                // Follow to correct child node.
                if direction == NodeDirection::RIGHT {
                    best_points = Self::nearest_neighbour(right_node.unwrap(), max_distance_sq, query_point, best_points, k);

                    /*
                     * IN Case: we missed.
                     * We may need to check the other side of the tree. If the other side is closer than the radius
                     */
                    if !left_node.is_none() {
                        distance_to_op_side = query_point.distance_to(left_node.unwrap().point.as_ref().unwrap().as_ref());
                        if distance_to_op_side < max_distance_sq { direction = NodeDirection::LEFT };
                    }
                }

                else if direction == NodeDirection::LEFT {
                    best_points = Self::nearest_neighbour(left_node.unwrap(), max_distance_sq, query_point, best_points, k);
                    if !right_node.is_none() {
                        distance_to_op_side = query_point.distance_to(right_node.unwrap().point.as_ref().unwrap().as_ref());
                        if distance_to_op_side < max_distance_sq { direction = NodeDirection::RIGHT };
                    }
                }

                // Make sure we have to go to child node.
                if distance_to_op_side < max_distance_sq {
                    if direction == NodeDirection::LEFT {
                        best_points =Self::nearest_neighbour(left_node.unwrap(), max_distance_sq, query_point, best_points, k);
                    } else if direction == NodeDirection::RIGHT {
                        best_points = Self::nearest_neighbour(right_node.unwrap(), max_distance_sq, query_point, best_points, k);
                    }
                }

                return best_points;
            }
        }

        best_points
    }

    fn direction(query_point: &P, node_point: &P, axis: usize) -> NodeDirection{
        let node_coord = node_point.get_coordinate();
        let q_coord = query_point.get_coordinate();

        // If Query point is greater than current point then go right.
        if q_coord[axis] - node_coord[axis] > 0f32 {
            NodeDirection::RIGHT
        }

        // If Query point is greater than current point then go left.
        else {
            NodeDirection::LEFT
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare_points() {
        let point_a = Point3D::new(1.0, 2.0, 3.0);
        let point_b = Point3D::new(2.0, 2.0, 3.0);
        let point_c = Point3D::new(1.0, 3.0, 3.0);

        assert_eq!(KDTree::sorting_point(&point_b, &point_a, 0).is_ok(), true);
        assert_eq!(KDTree::sorting_point(&point_b, &point_a, 0).unwrap(), Ordering::Greater);

        assert_eq!(KDTree::sorting_point(&point_a, &point_b, 1).is_ok(), true);
        assert_eq!(KDTree::sorting_point(&point_a, &point_b, 1).unwrap(), Ordering::Equal);

        assert_eq!(KDTree::sorting_point(&point_a, &point_c, 1).is_ok(), true);
        assert_eq!(KDTree::sorting_point(&point_a, &point_c, 1).unwrap(), Ordering::Less);

        assert_eq!(KDTree::sorting_point(&point_c, &point_a, 1).is_ok(), true);
        assert_eq!(KDTree::sorting_point(&point_c, &point_a, 1).unwrap(), Ordering::Greater);
;
        assert_eq!(KDTree::sorting_point(&point_a, &point_b, 2).is_ok(), true);
        assert_eq!(KDTree::sorting_point(&point_a, &point_b, 2).unwrap(), Ordering::Equal);
    }

    #[test]
    fn test_operation_point_list() {
        let mut points = vec![
            Point3D::new(1.0, 2.0, 3.0),
            Point3D::new(4.0, 5.0, 6.0),
            Point3D::new(7.0, 8.0, 9.0),
        ];

        let median = points.len() / 2;

        let left_points = KDTree::operation_point_list(&points, median, NodeDirection::LEFT);
        let right_points = KDTree::operation_point_list(&points, median, NodeDirection::RIGHT);

        assert_eq!(left_points, &points[..median]);
        assert_eq!(right_points, &points[median+1..]);
    }

    #[test]
    fn test_build_kd_tree() {
        let mut points = vec![
            Point3D::new(1.0, 2.0, 3.0),
            Point3D::new(4.0, 5.0, 6.0),
            Point3D::new(7.0, 8.0, 9.0),
            Point3D::new(2.0, 3.0, 4.0),
            Point3D::new(5.0, 6.0, 7.0),
            Point3D::new(8.0, 9.0, 10.0),
        ];


        let root = KDTree::create_kd_tree(&mut points, 0, 3).unwrap();
        root.point.as_ref().map(|point| assert_eq!(point, &Rc::new(Point3D::new(5.0, 6.0, 7.0))));

        if let Some(right) = &root.right {
           right.point.as_ref().map(
               |point| assert_eq!(point, &Rc::new(Point3D::new(8.0, 9.0, 10.0)))
           );

            right.left.as_ref().map(
                |leftt|
                    leftt.point.as_ref().map(
                        |point| assert_eq!(point, &Rc::new(Point3D::new(7.0, 8.0, 9.0)))
                    )
            );
        }

        if let Some(left) = &root.left {
            left.point.as_ref().map(
                |point| assert_eq!(point, &Rc::new(Point3D::new(2.0, 3.0, 4.0)))
            );

            left.left.as_ref().map(
                |leftt|
                    leftt.point.as_ref().map(
                        |point| assert_eq!(point, &Rc::new(Point3D::new(1.0, 2.0, 3.0)))
                    )
            );

            left.right.as_ref().map(
                |right|
                    right.point.as_ref().map(
                        |point| assert_eq!(point, &Rc::new(Point3D::new(4.0, 5.0, 6.0)))
                    )
            );
        }
    }

    #[test]
    fn test_find_closest() {

        let mut points = vec![
            Point3D::new(1.0, 1.0, 1.0),
            Point3D::new(2.0, 2.0, 2.0),
            Point3D::new(3.0, 3.0, 3.0),
            Point3D::new(4.0, 4.0, 4.0),
            Point3D::new(5.0, 5.0, 5.0)
        ];

        let root = KDTree::create_kd_tree(&mut points, 0, 3).unwrap();
        let query_point = Point3D::new(0.0, 0.0, 0.0);

        let point_limit = 2;

        let result = root.find_closest(
            &query_point, 3, point_limit
        );

        assert!(result.is_some());
        let best_points = result.unwrap();

        assert_eq!(best_points.len(), point_limit);
        assert_eq!(best_points[0].1, &Point3D::new(1.0, 1.0, 1.0));
        assert_eq!(best_points[1].1, &Point3D::new(2.0, 2.0, 2.0));
    }
}
