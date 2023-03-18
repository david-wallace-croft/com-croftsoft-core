// =============================================================================
//! - Unit tests for the A* algorithm associated functions
//!
//! # Metadata
//! - Copyright: &copy; 2022-2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2022-11-01
//! - Updated: 2023-03-18
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

#[cfg(test)]
use crate::ai::astar::structures::NodeInfo;
#[cfg(test)]
use crate::math::geom::point_2dd::Point2DD;

#[cfg(test)]
const TEST_NODE_0: Point2DD = Point2DD {
  x: 0.0,
  y: 0.0,
};

#[test]
fn test_new() {
  assert_eq!(
    NodeInfo::new(TEST_NODE_0),
    NodeInfo {
      cost_from_start: 0.0,
      node: TEST_NODE_0,
      total_cost: 0.0,
    }
  );
}
