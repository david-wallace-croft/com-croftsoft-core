// =============================================================================
//! - Unit tests for the A* algorithm associated functions
//!
//! # Metadata
//! - Copyright: &copy; 2022-2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2022-11-01
//! - Updated: 2023-06-25
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

#[cfg(test)]
use crate::ai::astar::structures::NodeInfo;

#[test]
fn test_default() {
  assert_eq!(
    NodeInfo::default(),
    NodeInfo {
      cost_from_start: 0.0,
      total_cost: 0.0,
    }
  );
}
