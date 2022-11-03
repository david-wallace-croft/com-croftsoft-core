// =============================================================================
//! - Unit tests for the A* algorithm trait implementations
//!
//! # Metadata
//! - Copyright: &copy; 2002 - 2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2022-11-03
//! - Rust since: 2022-11-02
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

#[cfg(test)]
use crate::{
  ai::astar::{
    structures::{GradientCartographer, GridCartographer, Rectangle},
    traits::Cartographer,
  },
  math::geom::structures::Point2DD,
};

#[cfg(test)]
const GOAL_NODE: Point2DD = Point2DD {
  x: 10.0,
  y: 0.0,
};
#[cfg(test)]
const NODE_FACTORY: Point2DD = Point2DD {
  x: 0.0,
  y: 0.0,
};
#[cfg(test)]
const SPACE_TESTER: Rectangle = Rectangle {
  x_max: 100.0,
  x_min: -100.0,
  y_max: 100.0,
  y_min: -100.0,
};

#[test]
fn test_estimate_to_goal_for_gradient_cartographer() {
  let test_subject_gradient_cartographer = GradientCartographer {
    directions: 8,
    goal_node: &GOAL_NODE,
    init_step_size: 1.0,
    node_factory: &NODE_FACTORY,
    space_tester: &SPACE_TESTER,
    start_node: &Point2DD::default(),
  };
  assert_eq!(
    test_subject_gradient_cartographer
      .estimate_cost_to_goal(&Point2DD::default()),
    10.0
  );
}

#[test]
fn test_estimate_to_goal_for_grid_cartographer() {
  let test_subject_grid_cartographer = GridCartographer {
    goal_node: &GOAL_NODE,
    node_factory: &NODE_FACTORY,
    space_tester: &SPACE_TESTER,
    step_size: 1.0,
  };
  assert_eq!(
    test_subject_grid_cartographer.estimate_cost_to_goal(&Point2DD::default()),
    10.0
  );
}
