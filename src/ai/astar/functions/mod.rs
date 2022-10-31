// =============================================================================
//! - Associated functions for the A* algorithm
//!
//! # Metadata
//! - Copyright: &copy; 2002 - 2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2022-10-31
//! - Rust since: 2022-10-28
//! - Java version: 2003-05-09
//! - Java since: 2002-04-21
//!
//! # History
//! - Adapted from the classes in the Java-based [`CroftSoft Core Library`]
//!   - com.croftsoft.core.ai.astar.AStar
//!
//! [`CroftSoft Core Library`]: https://www.croftsoft.com/library/code/
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::structures::NodeInfo;
use crate::math::geom::traits::PointXY;

impl<N: PointXY> NodeInfo<N> {
  pub fn new(node: N) -> Self {
    NodeInfo {
      cost_from_start: 0.0,
      node,
      total_cost: 0.0,
    }
  }
}
