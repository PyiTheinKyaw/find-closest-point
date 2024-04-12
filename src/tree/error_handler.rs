use std::cmp::Ordering;
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use crate::Point3D;
use crate::points::point::Point;
use crate::tree::Ikd::NodeDirection;

#[derive(Debug)]
pub enum ComparisonError {
    InvalidOrdering(String), // Custom error type for invalid input data
}

impl Display for ComparisonError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl ComparisonError {
    fn message(&self) -> &str {
        "ORDERING ERROR"
    }
}

impl Error for ComparisonError {}