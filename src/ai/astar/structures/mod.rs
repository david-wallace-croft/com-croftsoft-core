// =============================================================================
//! - Structures for the A* algorithm
//!
//! # Metadata
//! - Copyright: &copy; 2002 - 2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2022-10-26
//! - Rust since: 2022-10-22
//! - Java version: 2003-05-10
//! - Java since: 2002-04-21
//!
//! # History
//! - Adapted from the classes in the Java-based [`CroftSoft Core Library`]
//!   - com.croftsoft.core.ai.astar.GridCartographer
//!   - com.croftsoft.core.ai.astar.NodeInfo
//!
//! [`CroftSoft Core Library`]: https://www.croftsoft.com/library/code/
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::traits::{NodeFactory, SpaceTester};
use crate::math::geom::traits::PointXY;

/// Grid cartographer for continuous space.
/// The nodes are spaced equally apart in the eight cardinal directions.
pub struct GridCartographer<
  'f,
  'n,
  's,
  F: NodeFactory<N>,
  N: PointXY,
  S: SpaceTester<N>,
> {
  pub goal_node: &'n N,
  pub node_factory: &'f F,
  pub space_tester: &'s S,
  pub step_size: f64,
}

/// A* algorithm node information
pub struct NodeInfo<N: PointXY> {
  pub cost_from_start: f64,
  pub node: N,
  // pub parent_node_info: Option<NodeInfo<N>>,
  pub total_cost: f64,
}
