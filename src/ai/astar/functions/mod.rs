// =============================================================================
//! - Associated functions for the A* algorithm
//!
//! # Metadata
//! - Copyright: &copy; 2002 - 2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2022-11-14
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

mod test;

use super::{
  structures::{AStar, NodeInfo},
  traits::Cartographer,
};
use core::{f64::INFINITY, hash::Hash};
use std::collections::HashMap;

impl<C: Cartographer<N>, N: Eq + Hash> AStar<C, N> {
  pub fn new(cartographer: C) -> Self {
    AStar {
      best_node_info_option: None,
      best_total_cost: INFINITY,
      cartographer,
      goal_node_info_option: None,
      list_empty: false,
      node_to_node_info_map: HashMap::new(),
      node_to_parent_node_info_map: HashMap::new(),
      open_node_info_sorted_list: Vec::new(),
    }
  }
}

impl<N> NodeInfo<N> {
  pub fn new(node: N) -> Self {
    NodeInfo {
      cost_from_start: 0.0,
      node,
      total_cost: 0.0,
    }
  }
}
