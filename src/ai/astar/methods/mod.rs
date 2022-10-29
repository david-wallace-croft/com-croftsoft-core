// =============================================================================
//! - Methods for the A* algorithm
//!
//! # Metadata
//! - Copyright: &copy; 2002 - 2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2022-10-29
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

impl<'c, 'i, 'n, C: Cartographer<N>, N: Eq + Hash + PointXY>
  AStar<'c, 'i, 'n, C, N>
{
  pub fn get_first_step(&self) -> Option<N> {
    let mut node_info_option: Option<&NodeInfo<N>> = None;
    if self.goal_node_info_option.is_none() {
      node_info_option = Some(self.best_node_info);
    }
    let node_option: Option<N> = None;
    while node_info_option.is_some() {
      let node_info: &NodeInfo<N> = node_info_option.unwrap();
      let parent_node_info_option: Option<&NodeInfo<N>> =
        node_info.parent_node_info_option;
    }
    // TODO: left off here
    node_option
  }

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
