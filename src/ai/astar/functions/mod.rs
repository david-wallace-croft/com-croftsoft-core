// =============================================================================
//! - Associated functions for the A* algorithm
//!
//! # Metadata
//! - Copyright: &copy; 2022-2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Java created: 2002-04-21
//! - Java updated: 2003-05-09
//! - Rust created: 2022-10-28
//! - Rust updated: 2023-07-23
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

use super::structures::AStar;
use core::f64::INFINITY;
use core::hash::Hash;
use std::collections::HashMap;
use std::collections::VecDeque;

impl<N: Eq + Hash> Default for AStar<N> {
  fn default() -> Self {
    AStar {
      best_node_option: None,
      best_total_cost: INFINITY,
      goal_node_option: None,
      list_empty: false,
      node_to_node_info_map: HashMap::new(),
      node_to_parent_node_map: HashMap::new(),
      open_node_sorted_list: VecDeque::new(),
    }
  }
}
