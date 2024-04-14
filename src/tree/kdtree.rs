use std::cell::{Ref, RefCell, RefMut};
use std::cmp::{Ordering, PartialEq};
use std::fmt::Debug;
use std::rc::Rc;
use crate::points::point::{Point, Point3D};
use crate::tree::error_handler::ComparisonError;
use super::Ikd::{IKDTree, NodeDirection};

#[derive(Debug)]
pub struct KDTree<P>
{
    pub point: P,
    depth: usize,
    pub left: Option<Box<KDTree<P>>>,
    pub right: Option<Box<KDTree<P>>>,
}



/**
Implementation of KDTree
**/
impl<P> IKDTree<P> for KDTree<P>
    where P: Point<Point3D> + Copy + PartialEq + Debug
{
    type Output = KDTree<P>;

    fn new (point: P, depth: usize) -> Self::Output {
        KDTree { point, depth, left: None, right: None }
    }

    fn set_child_node(&mut self, node: Self::Output, direction: &NodeDirection) {
        match direction {
            NodeDirection::LEFT => self.left = Some(Box::new(node)),
            NodeDirection::RIGHT => self.right = Some(Box::new(node)),
            _ => ()
        }
    }

    fn create_kd_tree(points: &mut RefCell<&[P]>, depth: usize, k: usize) -> Result<Box<Self::Output>, String>
    {
        if points.borrow().len() == 0 {
            return Err(String::from("KDTreeBuildError: point len is zero."));
        }

        // Following code will init an KDTree object with zero value.
        let kd_tree = Self::build_kd_tree(
            points,
            k,
            depth
        );

        Ok(Box::new(kd_tree))
    }

    fn build_kd_tree
    (
        sorted_points : &mut RefCell<&[P]>,
        k: usize,
        depth: usize
    ) -> Self::Output
    {
        let axis = depth % k;

        // In order to get almost perfect balance tree, we have to sort it first.
        let sorted_list = Self::multi_dimensional_sort(sorted_points, axis);

        // find the median
        let median = sorted_points.borrow().len() / 2;

        // Create for current node position.
        let mut current_node = Self::new(
            sorted_points.borrow()[median],
            depth
        );

        // Median 0 means there is no points left to operate.
        // If it's not 0, it's still point left turn into node.
        if median != 0 {
            let mut direction = NodeDirection::NOWHERE;

            // Calculate the direction
            // If Median is 1 and len is 2.
            if median == 1 && sorted_points.borrow().len() == 2 {
                // Only left node to create
                // Best case
                let mut point_slice = Self::operation_point_list(
                    sorted_points.borrow(),
                    median,
                    &NodeDirection::LEFT
                );

                let child_node = Self::build_kd_tree(&mut point_slice, k,depth + 1);
                current_node.set_child_node(child_node, &NodeDirection::LEFT);
            }
            else {
                // Else, we have to create both childs - left and right.
                // Average case
                for index in 0..2 {

                    if NodeDirection::LEFT as u8 == index {
                        direction = NodeDirection::LEFT;
                    }
                    if NodeDirection::RIGHT as u8 == index {
                        direction = NodeDirection::RIGHT;
                    }

                    // Prepare data
                    let mut point_slice = Self::operation_point_list(
                        sorted_points.borrow(),
                        median,
                        &direction
                    );


                    // Create Child node according to direction.
                    let child_node = Self::build_kd_tree(&mut point_slice, k,depth + 1);
                    current_node.set_child_node(child_node, &direction);
                }
            }
        }

        // Return current node;
        current_node
    }

    fn multi_dimensional_sort<'a>(list: &'a mut RefCell<&'a [P]>, axis: usize) -> &'a mut RefCell<&'a [P]>
    {
        let mut data = list.borrow_mut().clone(); // Clone the data from RefCell
        data.sort_by(|a, b| {

            let a_coord = a.get_coordinate();
            let b_coord = b.get_coordinate();

            if axis == 0 {a_coord[0].partial_cmp(&b_coord[0]).unwrap()}
            else if axis == 1 {a_coord[1].partial_cmp(&b_coord[1]).unwrap()}
            else {a_coord[2].partial_cmp(&b_coord[2]).unwrap()}
        });

        *list.borrow_mut() = data;

        list
    }

    fn sorting_nearest(
        n_point_a: &(f32, &P),
        n_point_b: &(f32, &P),
    ) -> Result<Ordering, ComparisonError> {
        Ok(n_point_a.0.partial_cmp(&n_point_b.0).unwrap())
    }

    fn operation_point_list
    <'kdp>
    (
        points: Ref<&'kdp [P]>,
        median: usize,
        direction: &NodeDirection
    ) -> RefCell<&'kdp [P]>
    {
        if direction == NodeDirection::LEFT {
            RefCell::new(&points[..median])
        }
        else {
            RefCell::new(&points[median+1..])
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
        let point = &node.point;

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
                        distance_to_op_side = query_point.distance_to(&left_node.unwrap().point);
                        if distance_to_op_side < max_distance_sq { direction = NodeDirection::LEFT };
                    }
                }

                else if direction == NodeDirection::LEFT {
                    best_points = Self::nearest_neighbour(left_node.unwrap(), max_distance_sq, query_point, best_points, k);
                    if !right_node.is_none() {
                        distance_to_op_side = query_point.distance_to(&right_node.unwrap().point);
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
    fn test_operation_point_list() {
        let mut points = [
            Point3D::new(1.0, 2.0, 3.0),
            Point3D::new(4.0, 5.0, 6.0),
            Point3D::new(7.0, 8.0, 9.0),
        ];

        let median = points.len() / 2;

        let left_points = KDTree::operation_point_list(&mut points, median, &NodeDirection::LEFT);
        let right_points = KDTree::operation_point_list(&mut points, median, &NodeDirection::RIGHT);

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
