// =============================================================================
//! - Unit tests for the A* algorithm trait implementations
//!
//! # Metadata
//! - Copyright: &copy; 2002 - 2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2022-11-05
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
const TOLERANCE: f64 = 0.01;

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
const ORIGIN_NODE: Point2DD = Point2DD {
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
fn test_estimate_cost_to_goal_for_gradient_cartographer() {
  assert_eq!(
    TEST_SUBJECT_GRADIENT_CARTOGRAPHER.estimate_cost_to_goal(&ORIGIN_NODE),
    DISTANCE_TO_GOAL
  );
}

#[test]
fn test_estimate_cost_to_goal_for_grid_cartographer() {
  assert_eq!(
    TEST_SUBJECT_GRID_CARTOGRAPHER.estimate_cost_to_goal(&ORIGIN_NODE),
    DISTANCE_TO_GOAL
  );
}

#[test]
fn test_get_adjacent_nodes_for_gradient_cartographer() {
  let mut actual_adjacent_nodes =
    TEST_SUBJECT_GRADIENT_CARTOGRAPHER.get_adjacent_nodes(&ORIGIN_NODE);
  let expected_adjacent_nodes = [
    Point2DD {
      x: 1.0,
      y: 0.0,
    },
    Point2DD {
      x: 0.707,
      y: 0.707,
    },
    Point2DD {
      x: 0.0,
      y: 1.0,
    },
    Point2DD {
      x: -0.707,
      y: 0.707,
    },
    Point2DD {
      x: -1.0,
      y: 0.0,
    },
    Point2DD {
      x: -0.707,
      y: -0.707,
    },
    Point2DD {
      x: 0.0,
      y: -1.0,
    },
    Point2DD {
      x: 0.707,
      y: -0.707,
    },
  ];
  assert_eq!(actual_adjacent_nodes.len(), expected_adjacent_nodes.len());
  for expected_adjacent_node in expected_adjacent_nodes {
    let mut found: Option<usize> = None;
    for (index, actual_adjacent_node) in
      actual_adjacent_nodes.iter().enumerate()
    {
      // TODO: implement a matches_closely() or is_near() method for Point2DD
      if (actual_adjacent_node.x - expected_adjacent_node.x).abs() <= TOLERANCE
        && (actual_adjacent_node.y - expected_adjacent_node.y).abs()
          <= TOLERANCE
      {
        found = Some(index);
        break;
      }
    }
    if let Some(index) = found {
      actual_adjacent_nodes.remove(index);
    } else {
      assert_eq!(actual_adjacent_nodes, expected_adjacent_nodes);
    }
  }
}

#[test]
fn test_get_adjacent_nodes_for_grid_cartographer() {
  assert_eq!(
    TEST_SUBJECT_GRID_CARTOGRAPHER.get_adjacent_nodes(&Point2DD::default()),
    vec![
      Point2DD {
        x: -1.0,
        y: -1.0
      },
      Point2DD {
        x: -1.0,
        y: 0.0
      },
      Point2DD {
        x: -1.0,
        y: 1.0
      },
      Point2DD {
        x: 0.0,
        y: -1.0
      },
      Point2DD {
        x: 0.0,
        y: 1.0
      },
      Point2DD {
        x: 1.0,
        y: -1.0
      },
      Point2DD {
        x: 1.0,
        y: 0.0
      },
      Point2DD {
        x: 1.0,
        y: 1.0
      },
    ],
  );
}

#[test]
fn test_get_cost_to_adjacent_node_for_gradient_cartographer() {
  assert_eq!(
    TEST_SUBJECT_GRADIENT_CARTOGRAPHER
      .get_cost_to_adjacent_node(&Point2DD::default(), &GOAL_NODE),
    DISTANCE_TO_GOAL,
  );
}

#[test]
fn test_get_cost_to_adjacent_node_for_grid_cartographer() {
  assert_eq!(
    TEST_SUBJECT_GRID_CARTOGRAPHER
      .get_cost_to_adjacent_node(&Point2DD::default(), &GOAL_NODE),
    DISTANCE_TO_GOAL,
  );
}

#[test]
fn test_is_goal_node_for_gradient_cartographer() {
  assert!(TEST_SUBJECT_GRADIENT_CARTOGRAPHER.is_goal_node(&GOAL_NODE));
  assert!(!TEST_SUBJECT_GRADIENT_CARTOGRAPHER.is_goal_node(&ORIGIN_NODE));
}

#[test]
fn test_is_goal_node_for_grid_cartographer() {
  assert!(TEST_SUBJECT_GRID_CARTOGRAPHER.is_goal_node(&GOAL_NODE));
  assert!(!TEST_SUBJECT_GRID_CARTOGRAPHER.is_goal_node(&ORIGIN_NODE));
}
