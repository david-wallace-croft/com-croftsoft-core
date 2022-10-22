// =============================================================================
//! - Structures for the A* algorithm
//!
//! # Metadata
//! - Copyright: &copy; 2002 - 2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2022-10-22
//! - Rust since: 2022-10-22
//! - Java version: 2003-05-09
//! - Java since: 2002-04-21
//!
//! # History
//! - Adapted from the classes in the Java-based [`CroftSoft Core Library`]
//!   - com.croftsoft.core.ai.astar.NodeInfo
//!
//! [`CroftSoft Core Library`]: https://www.croftsoft.com/library/code/
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

/// A* algorithm node information
pub struct NodeInfo<N> {
  pub cost_from_start: f64,
  pub node: N,
  // pub parent_node_info: Option<NodeInfo<N>>,
  pub total_cost: f64,
}
