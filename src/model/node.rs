use std::fmt::Debug;
use crate::model::direction::NodeDirection;
#[derive(Debug)]
pub struct Node<T> {
    value: T,
    depth: usize,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>
}

impl<T> Node<T>
{
    pub fn get_empty_node(value: T,  depth: usize) -> Node<T> {
        Node {
            value,
            depth,
            left: None,
            right: None
        }
    }

    pub fn set_child_node(&mut self, node: Self, direction: NodeDirection) {
        if direction == NodeDirection::LEFT {
            self.left = Some(Box::new(node));
        }
        else if direction == NodeDirection::RIGHT {
            self.right = Some(Box::new(node));
        }
    }
}