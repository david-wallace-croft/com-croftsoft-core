// =============================================================================
//! - Methods for the A* algorithm
//!
//! # Metadata
//! - Copyright: &copy; 2002 - 2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2022-10-28
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

use core::f64::INFINITY;
use core::hash::Hash;
use std::collections::HashMap;

use super::{
  structures::{AStar, NodeInfo},
  traits::Cartographer,
};
use crate::math::geom::traits::PointXY;

impl<'c, 'n, C: Cartographer<N>, N: Eq + Hash + PointXY> AStar<'c, 'n, C, N> {
  pub fn is_goal_found(&self) -> bool {
    self.goal_node_info_option.is_some()
  }

  pub fn reset(
    &mut self,
    start_node: &'n N,
  ) {
    self.goal_node_info_option = None;
    self.list_empty = false;
    self.open_node_info_sorted_list = Vec::new();
    self.node_to_node_info_map = HashMap::new();
    let start_node_info = NodeInfo::new(start_node);
    self.node_to_node_info_map.insert(start_node, start_node_info);
    self.open_node_info_sorted_list.push(start_node);
    self.best_total_cost = INFINITY;
  }
}
