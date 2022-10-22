// =============================================================================
//! - Operation implementations for the A* algorithm trait implementations
//!
//! # Metadata
//! - Copyright: &copy; 2002 - 2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2022-10-22
//! - Rust since: 2022-10-22
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use std::cmp::Ordering;

use crate::ai::astar::structures::NodeInfo;

#[cfg(test)]
const NODE_INFO_0: NodeInfo<&str> = NodeInfo {
  cost_from_start: 0.0,
  node: "alpha",
  // parent_node_info: None,
  total_cost: 0.0,
};

#[cfg(test)]
const NODE_INFO_1: NodeInfo<&str> = NodeInfo {
  cost_from_start: 0.0,
  node: "beta",
  // parent_node_info: None,
  total_cost: 1.0,
};

#[test]
fn test_eq() {
  assert!(NODE_INFO_0 != NODE_INFO_1);
}

#[test]
fn test_ge() {
  assert!(NODE_INFO_1 >= NODE_INFO_0);
}

#[test]
fn test_gt() {
  assert!(NODE_INFO_1 > NODE_INFO_0);
}

#[test]
fn test_le() {
  assert!(NODE_INFO_0 <= NODE_INFO_1);
}

#[test]
fn test_lt() {
  assert!(NODE_INFO_0 < NODE_INFO_1);
}

#[test]
fn test_partial_cmp() {
  assert_eq!(NODE_INFO_0.partial_cmp(&NODE_INFO_1), Some(Ordering::Less));
}
