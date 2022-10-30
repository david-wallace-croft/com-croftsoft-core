// =============================================================================
//! - Methods for the A* algorithm
//!
//! # Metadata
//! - Copyright: &copy; 2002 - 2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2022-10-30
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
      if parent_node_info_option.is_some() {
        node_info_option = parent_node_info_option;
      }
    }
    node_option
  }

  pub fn get_path(&self) -> Vec<&N> {
    let mut path_list = Vec::new();
    let mut node_info_option: Option<&NodeInfo<N>> = None;
    if self.goal_node_info_option.is_none() {
      node_info_option = Some(self.best_node_info);
    }
    while node_info_option.is_some() {
      let node_info: &NodeInfo<N> = node_info_option.unwrap();
      let parent_node_info_option: Option<&NodeInfo<N>> =
        node_info.parent_node_info_option;
      if parent_node_info_option.is_some() {
        path_list.insert(0, node_info.node);
      }
      node_info_option = parent_node_info_option;
    }
    path_list
  }

  pub fn is_goal_found(&self) -> bool {
    self.goal_node_info_option.is_some()
  }

  pub fn loop_once(&mut self) -> bool {
    if self.open_node_info_sorted_list.is_empty() {
      self.list_empty = true;
      return false;
    }
    let node_info: NodeInfo<N> = self.open_node_info_sorted_list.remove(0);
    let node: &N = node_info.node;
    if self.cartographer.is_goal_node(node) {
      if self.goal_node_info_option.is_none()
        || node_info.cost_from_start
          < self.goal_node_info_option.as_mut().unwrap().cost_from_start
      {
        self.goal_node_info_option = Some(node_info);
      }
      return false;
    }
    // TODO: left off here
    true
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
    self.open_node_info_sorted_list.push(start_node_info);
    self.best_total_cost = INFINITY;
  }
}
