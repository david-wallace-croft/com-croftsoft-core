// =============================================================================
//! - Associated functions for the A* algorithm
//!
//! # Metadata
//! - Copyright: &copy; 2022-2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Java created: 2002-04-21
//! - Java updated: 2003-05-09
//! - Rust created: 2022-10-28
//! - Rust updated: 2023-04-16
//!
//! # History
//! - Adapted from the classes in the Java-based [`CroftSoft Core Library`]
//!   - com.croftsoft.core.ai.astar.AStar
//!
//! [`CroftSoft Core Library`]: https://www.croftsoft.com/library/code/
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

#[cfg(test)]
mod test;

use super::{
  structures::{AStar, NodeInfo},
  traits::Cartographer,
};
use core::{cell::RefCell, f64::INFINITY, hash::Hash};
use std::collections::HashMap;
use std::rc::Rc;

impl<C: Cartographer<N>, N: Eq + Hash> AStar<C, N> {
  pub fn new(cartographer: Rc<RefCell<C>>) -> Self {
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
