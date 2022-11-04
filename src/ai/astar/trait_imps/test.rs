// =============================================================================
//! - Unit tests for the A* algorithm trait implementations
//!
//! # Metadata
//! - Copyright: &copy; 2002 - 2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2022-11-04
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
const DISTANCE_TO_GOAL: f64 = 4.0;
#[cfg(test)]
const GOAL_NODE: Point2DD = Point2DD {
  x: DISTANCE_TO_GOAL,
  y: 0.0,
};
#[cfg(test)]
const NODE_FACTORY: Point2DD = Point2DD {
  x: 0.0,
  y: 0.0,
};
#[cfg(test)]
const SPACE_TESTER: Rectangle = Rectangle {
  x_max: 10.0,
  x_min: -10.0,
  y_max: 10.0,
  y_min: -10.0,
};
#[cfg(test)]
const START_NODE: Point2DD = Point2DD {
  x: 0.0,
  y: 0.0,
};
#[cfg(test)]
const STEP_SIZE: f64 = 1.0;
#[cfg(test)]
const TEST_SUBJECT_GRADIENT_CARTOGRAPHER: GradientCartographer<
  Point2DD,
  Point2DD,
  Rectangle,
> = GradientCartographer {
  directions: 8,
  goal_node: &GOAL_NODE,
  init_step_size: STEP_SIZE,
  node_factory: &NODE_FACTORY,
  space_tester: &SPACE_TESTER,
  start_node: &START_NODE,
};
#[cfg(test)]
const TEST_SUBJECT_GRID_CARTOGRAPHER: GridCartographer<
  Point2DD,
  Point2DD,
  Rectangle,
> = GridCartographer {
  goal_node: &GOAL_NODE,
  node_factory: &NODE_FACTORY,
  space_tester: &SPACE_TESTER,
  step_size: STEP_SIZE,
};

#[test]
fn test_estimate_to_goal_for_gradient_cartographer() {
  assert_eq!(
    TEST_SUBJECT_GRADIENT_CARTOGRAPHER
      .estimate_cost_to_goal(&Point2DD::default()),
    DISTANCE_TO_GOAL
  );
}

#[test]
fn test_estimate_to_goal_for_grid_cartographer() {
  assert_eq!(
    TEST_SUBJECT_GRID_CARTOGRAPHER.estimate_cost_to_goal(&Point2DD::default()),
    DISTANCE_TO_GOAL
  );
}

// TODO: tests for get_adjacent_nodes()

#[test]
fn test_get_cost_to_adjacent_node_for_gradient_cartographer() {
  assert_eq!(
    TEST_SUBJECT_GRADIENT_CARTOGRAPHER
      .get_cost_to_adjacent_node(&Point2DD::default(), &GOAL_NODE),
    DISTANCE_TO_GOAL,
  );
}

// TODO: pass nodes to cartographers by copy instead of reference
