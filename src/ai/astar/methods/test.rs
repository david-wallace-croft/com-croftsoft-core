// =============================================================================
//! - Unit tests for the A* algorithm associated functions
//!
//! # Metadata
//! - Copyright: &copy; 2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2022-11-09
//! - Rust since: 2022-11-08
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

#[cfg(test)]
use core::f64::INFINITY;
#[cfg(test)]
use std::collections::HashMap;

#[cfg(test)]
use crate::{
  ai::astar::{
    constants::test::{
      TEST_GOAL_NODE, TEST_ORIGIN_NODE, TEST_SUBJECT_GRID_CARTOGRAPHER,
    },
    structures::{AStar, GridCartographer, NodeInfo, Rectangle},
  },
  math::geom::structures::Point2DD,
};

#[cfg(test)]
const TEST_BEST_NODE_INFO: NodeInfo<Point2DD> = NodeInfo {
  cost_from_start: 0.0,
  node: Point2DD {
    x: 0.0,
    y: 0.0,
  },
  total_cost: 0.0,
};
#[cfg(test)]
const TEST_GOAL_NODE_INFO: NodeInfo<Point2DD> = NodeInfo {
  cost_from_start: 0.0,
  node: TEST_GOAL_NODE,
  total_cost: 0.0,
};

// TODO
// #[test]
// fn test_get_first_step_for_grid_cartographer() {
//   let testSubjectAStar: AStar<
//     GridCartographer<Point2DD, Point2DD, Rectangle>,
//     Point2DD,
//   > = AStar {
//     best_node_info: TEST_BEST_NODE_INFO,
//     best_total_cost: 0.0,
//     cartographer: TEST_SUBJECT_GRID_CARTOGRAPHER,
//     goal_node_info_option: Some(TEST_GOAL_NODE_INFO),
//     list_empty: true,
//     node_to_node_info_map: HashMap::new(),
//     node_to_parent_node_info_map: HashMap::new(),
//     open_node_info_sorted_list: vec![],
//   };
//   let first_step_option = testSubjectAStar.get_first_step();
// }

#[test]
fn test_reset() {
  let mut test_subject_astar: AStar<
    GridCartographer<Point2DD, Rectangle>,
    Point2DD,
  > = AStar {
    best_node_info: TEST_BEST_NODE_INFO,
    best_total_cost: 0.0,
    cartographer: TEST_SUBJECT_GRID_CARTOGRAPHER,
    goal_node_info_option: Some(TEST_GOAL_NODE_INFO),
    list_empty: true,
    node_to_node_info_map: HashMap::new(),
    node_to_parent_node_info_map: HashMap::new(),
    open_node_info_sorted_list: vec![],
  };
  test_subject_astar.reset(TEST_ORIGIN_NODE);
  let AStar {
    best_node_info: _,
    best_total_cost,
    cartographer: _,
    goal_node_info_option,
    list_empty,
    node_to_node_info_map,
    node_to_parent_node_info_map,
    open_node_info_sorted_list,
  } = test_subject_astar;
  assert_eq!(best_total_cost, INFINITY);
  assert_eq!(goal_node_info_option, None);
  assert!(!list_empty);
  assert_eq!(
    open_node_info_sorted_list,
    vec![NodeInfo::new(TEST_ORIGIN_NODE)],
  );
  let mut expected_node_to_node_info_map = HashMap::new();
  expected_node_to_node_info_map
    .insert(TEST_ORIGIN_NODE, NodeInfo::new(TEST_ORIGIN_NODE));
  assert_eq!(node_to_node_info_map, expected_node_to_node_info_map);
  assert!(node_to_parent_node_info_map.is_empty());
}
