// =============================================================================
//! - Unit tests for the A* algorithm associated functions
//!
//! # Metadata
//! - Copyright: &copy; 2022-2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2022-11-08
//! - Updated: 2023-07-26
//!
//! [`CroftSoft Inc`]: https://www.CroftSoft.com/
//! [`David Wallace Croft`]: https://www.CroftSoft.com/people/david/
// =============================================================================

#[cfg(test)]
use crate::ai::astar::constants::test::{TEST_BEST_NODE, TEST_GOAL_NODE};
#[cfg(test)]
use crate::{
  ai::astar::{
    constants::test::{TEST_ORIGIN_NODE, TEST_SUBJECT_GRID_CARTOGRAPHER},
    structures::{AStar, NodeInfo},
  },
  math::geom::point_2dd::Point2DD,
};
#[cfg(test)]
use core::f64::INFINITY;
#[cfg(test)]
use std::collections::HashMap;
#[cfg(test)]
use std::collections::VecDeque;

#[test]
fn test_get_first_step() {
  let test_subject_astar = AStar::<Point2DD>::default();
  let first_step_option = test_subject_astar.get_first_step();
  assert_eq!(first_step_option, None);
}

#[test]
fn test_get_path() {
  let test_subject_astar = AStar::<Point2DD>::default();
  let path = test_subject_astar.get_path();
  assert_eq!(path, Vec::<Point2DD>::default());
}

#[test]
fn test_is_goal_found() {
  let test_subject_astar = AStar::<Point2DD>::default();
  assert!(!test_subject_astar.is_goal_found());
}

#[test]
fn test_loop_once() {
  let mut test_subject_astar: AStar<Point2DD> = AStar::default();
  assert!(!test_subject_astar.loop_once(&TEST_SUBJECT_GRID_CARTOGRAPHER));
}

#[test]
fn test_reset() {
  let mut test_subject_astar: AStar<Point2DD> = AStar {
    best_node_option: Some(TEST_BEST_NODE),
    best_total_cost: 0.0,
    goal_node_option: Some(TEST_GOAL_NODE),
    list_empty: true,
    node_to_node_info_map: HashMap::new(),
    node_to_parent_node_map: HashMap::new(),
    open_node_sorted_list: VecDeque::new(),
  };
  test_subject_astar.reset(TEST_ORIGIN_NODE);
  let AStar {
    best_node_option: _,
    best_total_cost,
    goal_node_option,
    list_empty,
    node_to_node_info_map,
    node_to_parent_node_map,
    open_node_sorted_list,
  } = test_subject_astar;
  assert_eq!(best_total_cost, INFINITY);
  assert_eq!(goal_node_option, None);
  assert!(!list_empty);
  let mut expected_open_node_sorted_list = VecDeque::new();
  expected_open_node_sorted_list.push_front(TEST_ORIGIN_NODE);
  assert_eq!(open_node_sorted_list, expected_open_node_sorted_list);
  let mut expected_node_to_node_info_map = HashMap::new();
  expected_node_to_node_info_map.insert(TEST_ORIGIN_NODE, NodeInfo::default());
  assert_eq!(node_to_node_info_map, expected_node_to_node_info_map);
  assert!(node_to_parent_node_map.is_empty());
}
