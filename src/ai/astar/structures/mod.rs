// =============================================================================
//! - Structures for the A* algorithm
//!
//! # Metadata
//! - Copyright: &copy; 2022-2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Java created: 2002-04-21
//! - Java updated: 2003-05-10
//! - Rust created: 2022-10-22
//! - Rust updated: 2023-04-16
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
use core::{cell::RefCell, hash::Hash};
use std::collections::HashMap;
use std::collections::VecDeque;
use std::rc::Rc;

pub struct AStar<C: Cartographer<N>, N: Eq + Hash> {
  pub best_node_option: Option<N>,
  pub best_total_cost: f64,
  pub cartographer: Rc<RefCell<C>>,
  pub goal_node_option: Option<N>,
  pub list_empty: bool,
  pub node_to_node_info_map: HashMap<N, NodeInfo>,
  pub node_to_parent_node_map: HashMap<N, N>,
  pub open_node_sorted_list: VecDeque<N>,
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

#[derive(Clone, Copy, Debug, Default)]
/// A* algorithm node information
pub struct NodeInfo {
  pub cost_from_start: f64,
  pub total_cost: f64,
}
