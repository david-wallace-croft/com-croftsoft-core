// =============================================================================
//! - Unit tests for the A* algorithm operations
//!
//! # Metadata
//! - Copyright: &copy; 2022-2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2022-10-22
//! - Updated: 2023-07-26
//!
//! [`CroftSoft Inc`]: https://www.CroftSoft.com/
//! [`David Wallace Croft`]: https://www.CroftSoft.com/people/david/
// =============================================================================

use crate::ai::astar::structures::NodeInfo;
use std::cmp::Ordering;

#[cfg(test)]
const NODE_INFO_0: NodeInfo = NodeInfo {
  cost_from_start: 0.0,
  total_cost: 0.0,
};

#[cfg(test)]
const NODE_INFO_1: NodeInfo = NodeInfo {
  cost_from_start: 0.0,
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
