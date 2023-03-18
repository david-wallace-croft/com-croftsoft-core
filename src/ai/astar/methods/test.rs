// =============================================================================
//! - Unit tests for the A* algorithm associated functions
//!
//! # Metadata
//! - Copyright: &copy; 2022-2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2022-11-08
//! - Updated: 2023-03-18
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

#[cfg(test)]
use crate::{
  ai::astar::{
    constants::test::{
      TEST_BEST_NODE_INFO, TEST_GOAL_NODE_INFO, TEST_ORIGIN_NODE,
      TEST_SUBJECT_GRID_CARTOGRAPHER,
    },
    structures::{AStar, GridCartographer, NodeInfo},
  },
  math::geom::point_2dd::Point2DD,
};
#[cfg(test)]
use core::f64::INFINITY;
#[cfg(test)]
use std::collections::HashMap;

#[test]
fn test_get_first_step() {
  let test_subject_astar: AStar<GridCartographer<Point2DD>, Point2DD> =
    AStar::new(TEST_SUBJECT_GRID_CARTOGRAPHER);
  let first_step_option = test_subject_astar.get_first_step();
  assert_eq!(first_step_option, None);
}

#[test]
fn test_get_path() {
  let test_subject_astar: AStar<GridCartographer<Point2DD>, Point2DD> =
    AStar::new(TEST_SUBJECT_GRID_CARTOGRAPHER);
  let path = test_subject_astar.get_path();
  assert_eq!(path, Vec::<Point2DD>::default());
}

#[test]
fn test_is_goal_found() {
  let test_subject_astar: AStar<GridCartographer<Point2DD>, Point2DD> =
    AStar::new(TEST_SUBJECT_GRID_CARTOGRAPHER);
  assert!(!test_subject_astar.is_goal_found());
}

#[test]
fn test_loop_once() {
  let mut test_subject_astar: AStar<GridCartographer<Point2DD>, Point2DD> =
    AStar::new(TEST_SUBJECT_GRID_CARTOGRAPHER);
  assert!(!test_subject_astar.loop_once());
}

#[test]
fn test_reset() {
  let mut test_subject_astar: AStar<GridCartographer<Point2DD>, Point2DD> =
    AStar {
      best_node_info_option: Some(TEST_BEST_NODE_INFO),
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
    best_node_info_option: _,
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
