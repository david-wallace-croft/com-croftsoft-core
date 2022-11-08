// =============================================================================
//! - Unit tests for the A* algorithm trait implementations
//!
//! # Metadata
//! - Copyright: &copy; 2002 - 2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2022-11-08
//! - Rust since: 2022-11-02
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

#[cfg(test)]
use core::cmp::Ordering;

#[cfg(test)]
use crate::{
  ai::astar::{
    constants::test::{
      TEST_DISTANCE_TO_BORDER, TEST_DISTANCE_TO_GOAL, TEST_GOAL_NODE,
      TEST_NODE_FACTORY, TEST_ORIGIN_NODE, TEST_SPACE_TESTER,
      TEST_SUBJECT_GRADIENT_CARTOGRAPHER, TEST_SUBJECT_GRID_CARTOGRAPHER,
      TEST_TOLERANCE,
    },
    structures::NodeInfo,
    traits::{Cartographer, NodeFactory, SpaceTester},
  },
  math::geom::structures::Point2DD,
};

#[test]
fn test_cmp() {
  let node_info_0 = NodeInfo {
    cost_from_start: 0.0,
    node: Point2DD::default(),
    total_cost: 0.0,
  };
  let node_info_1 = NodeInfo {
    cost_from_start: 0.0,
    node: Point2DD::default(),
    total_cost: 1.0,
  };
  assert_eq!(node_info_0.cmp(&node_info_0), Ordering::Equal);
  assert_eq!(node_info_0.cmp(&node_info_1), Ordering::Less);
  assert_eq!(node_info_1.cmp(&node_info_0), Ordering::Greater);
}

#[test]
fn test_estimate_cost_to_goal_for_gradient_cartographer() {
  assert_eq!(
    TEST_SUBJECT_GRADIENT_CARTOGRAPHER.estimate_cost_to_goal(&TEST_ORIGIN_NODE),
    TEST_DISTANCE_TO_GOAL
  );
}

#[test]
fn test_estimate_cost_to_goal_for_grid_cartographer() {
  assert_eq!(
    TEST_SUBJECT_GRID_CARTOGRAPHER.estimate_cost_to_goal(&TEST_ORIGIN_NODE),
    TEST_DISTANCE_TO_GOAL
  );
}

#[test]
fn test_get_adjacent_nodes_for_gradient_cartographer() {
  let mut actual_adjacent_nodes =
    TEST_SUBJECT_GRADIENT_CARTOGRAPHER.get_adjacent_nodes(&TEST_ORIGIN_NODE);
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
      if expected_adjacent_node.is_near(actual_adjacent_node, TEST_TOLERANCE) {
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
      .get_cost_to_adjacent_node(&Point2DD::default(), &TEST_GOAL_NODE),
    TEST_DISTANCE_TO_GOAL,
  );
}

#[test]
fn test_get_cost_to_adjacent_node_for_grid_cartographer() {
  assert_eq!(
    TEST_SUBJECT_GRID_CARTOGRAPHER
      .get_cost_to_adjacent_node(&Point2DD::default(), &TEST_GOAL_NODE),
    TEST_DISTANCE_TO_GOAL,
  );
}

#[test]
fn test_is_goal_node_for_gradient_cartographer() {
  assert!(TEST_SUBJECT_GRADIENT_CARTOGRAPHER.is_goal_node(&TEST_GOAL_NODE));
  assert!(!TEST_SUBJECT_GRADIENT_CARTOGRAPHER.is_goal_node(&TEST_ORIGIN_NODE));
}

#[test]
fn test_is_goal_node_for_grid_cartographer() {
  assert!(TEST_SUBJECT_GRID_CARTOGRAPHER.is_goal_node(&TEST_GOAL_NODE));
  assert!(!TEST_SUBJECT_GRID_CARTOGRAPHER.is_goal_node(&TEST_ORIGIN_NODE));
}

#[test]
fn test_is_space_available() {
  assert!(TEST_SPACE_TESTER.is_space_available(&TEST_GOAL_NODE));
  assert!(TEST_SPACE_TESTER.is_space_available(&Point2DD {
    x: TEST_DISTANCE_TO_BORDER,
    y: TEST_DISTANCE_TO_BORDER,
  }));
  assert!(!TEST_SPACE_TESTER.is_space_available(&Point2DD {
    x: TEST_DISTANCE_TO_BORDER + 1.0,
    y: 0.0,
  }));
}

#[test]
fn test_make_node() {
  assert_eq!(TEST_NODE_FACTORY.make_node(0.0, 0.0), Point2DD::default());
}
