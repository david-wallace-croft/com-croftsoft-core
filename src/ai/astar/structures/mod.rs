// =============================================================================
//! - Structures for the A* algorithm
//!
//! # Metadata
//! - Copyright: &copy; 2002 - 2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2022-11-14
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

use super::{
  traits::Cartographer,
  types::{IsSpaceAvailableFunction, MakeNodeFunction},
};
use core::hash::Hash;
use std::collections::HashMap;

pub struct AStar<C: Cartographer<N>, N: Eq + Hash> {
  pub best_node_info_option: Option<NodeInfo<N>>,
  pub best_total_cost: f64,
  pub cartographer: C,
  pub goal_node_info_option: Option<NodeInfo<N>>,
  pub list_empty: bool,
  pub node_to_node_info_map: HashMap<N, NodeInfo<N>>,
  pub node_to_parent_node_info_map: HashMap<N, NodeInfo<N>>,
  pub open_node_info_sorted_list: Vec<NodeInfo<N>>,
}

/// Gradient cartographer for continuous space.
/// The adjacent nodes are spaced farther apart as you move away from the
/// starting point.
pub struct GradientCartographer<N> {
  pub directions: u64,
  pub goal_node: N,
  pub init_step_size: f64,
  pub is_space_available_fn: IsSpaceAvailableFunction<N>,
  pub make_node_fn: MakeNodeFunction<N>,
  pub start_node: N,
}

/// Grid cartographer for continuous space.
/// The nodes are spaced equally apart in the eight cardinal directions.
pub struct GridCartographer<N> {
  pub goal_node: N,
  pub is_space_available_fn: IsSpaceAvailableFunction<N>,
  pub make_node_fn: MakeNodeFunction<N>,
  pub step_size: f64,
}

#[derive(Clone, Copy, Debug)]
/// A* algorithm node information
pub struct NodeInfo<N> {
  pub cost_from_start: f64,
  pub node: N,
  pub total_cost: f64,
}
