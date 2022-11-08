// =============================================================================
//! - Constants for the A* algorithm unit tests
//!
//! # Metadata
//! - Copyright: &copy; 2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2022-11-08
//! - Rust since: 2022-11-08
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

#[cfg(test)]
use crate::{
  ai::astar::structures::{GradientCartographer, GridCartographer, Rectangle},
  math::geom::structures::Point2DD,
};

#[cfg(test)]
pub const TEST_DISTANCE_TO_BORDER: f64 = 10.0;

#[cfg(test)]
pub const TEST_DISTANCE_TO_GOAL: f64 = 4.0;

#[cfg(test)]
pub const TEST_GOAL_NODE: Point2DD = Point2DD {
  x: TEST_DISTANCE_TO_GOAL,
  y: 0.0,
};

#[cfg(test)]
pub const TEST_NODE_FACTORY: Point2DD = Point2DD {
  x: 0.0,
  y: 0.0,
};

#[cfg(test)]
pub const TEST_ORIGIN_NODE: Point2DD = Point2DD {
  x: 0.0,
  y: 0.0,
};

#[cfg(test)]
pub const TEST_SPACE_TESTER: Rectangle = Rectangle {
  x_max: TEST_DISTANCE_TO_BORDER,
  x_min: -TEST_DISTANCE_TO_BORDER,
  y_max: TEST_DISTANCE_TO_BORDER,
  y_min: -TEST_DISTANCE_TO_BORDER,
};

#[cfg(test)]
pub const TEST_START_NODE: Point2DD = Point2DD {
  x: 0.0,
  y: 0.0,
};

#[cfg(test)]
pub const TEST_STEP_SIZE: f64 = 1.0;

#[cfg(test)]
pub const TEST_SUBJECT_GRADIENT_CARTOGRAPHER: GradientCartographer<
  Point2DD,
  Point2DD,
  Rectangle,
> = GradientCartographer {
  directions: 8,
  goal_node: TEST_GOAL_NODE,
  init_step_size: TEST_STEP_SIZE,
  node_factory: TEST_NODE_FACTORY,
  space_tester: TEST_SPACE_TESTER,
  start_node: TEST_START_NODE,
};

#[cfg(test)]
pub const TEST_SUBJECT_GRID_CARTOGRAPHER: GridCartographer<
  Point2DD,
  Point2DD,
  Rectangle,
> = GridCartographer {
  goal_node: TEST_GOAL_NODE,
  node_factory: TEST_NODE_FACTORY,
  space_tester: TEST_SPACE_TESTER,
  step_size: TEST_STEP_SIZE,
};

#[cfg(test)]
pub const TEST_TOLERANCE: f64 = 0.01;
