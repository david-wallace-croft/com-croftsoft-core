// =============================================================================
//! - Methods for the A* algorithm
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

use core::f64::INFINITY;
use core::hash::Hash;
use std::collections::HashMap;

use super::{
  structures::{AStar, NodeInfo},
  traits::Cartographer,
};

impl<C: Cartographer<N>, N: Copy + Eq + Hash> AStar<C, N> {
  pub fn get_first_step(&self) -> Option<N> {
    let mut node_info_option: Option<NodeInfo<N>> = self.goal_node_info_option;
    if node_info_option.is_none() {
      node_info_option = self.best_node_info_option;
    }
    let mut node_option: Option<N> = None;
    while node_info_option.is_some() {
      let node_info: NodeInfo<N> = node_info_option.unwrap();
      let parent_node_info_option: Option<NodeInfo<N>> =
        self.node_to_parent_node_info_map.get(&node_info.node).copied();
      if parent_node_info_option.is_some() {
        node_option = Some(node_info.node);
      }
      node_info_option = parent_node_info_option;
    }
    node_option
  }

  pub fn get_path(&self) -> Vec<N> {
    let mut path_list = Vec::new();
    let mut node_info_option: Option<NodeInfo<N>> = self.goal_node_info_option;
    if node_info_option.is_none() {
      node_info_option = self.best_node_info_option;
    }
    while node_info_option.is_some() {
      let node_info: NodeInfo<N> = node_info_option.unwrap();
      let parent_node_info_option: Option<NodeInfo<N>> =
        self.node_to_parent_node_info_map.get(&node_info.node).copied();
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
    let node: &N = &node_info.node;
    if self.cartographer.borrow().is_goal_node(node) {
      if self.goal_node_info_option.is_none()
        || node_info.cost_from_start
          < self.goal_node_info_option.as_mut().unwrap().cost_from_start
      {
        self.goal_node_info_option = Some(node_info);
      }
      return false;
    }
    let adjacent_nodes: Vec<N> =
      self.cartographer.borrow().get_adjacent_nodes(node);
    for adjacent_node in adjacent_nodes {
      let new_cost_from_start: f64 = node_info.cost_from_start
        + self
          .cartographer
          .borrow()
          .get_cost_to_adjacent_node(node, &adjacent_node);
      let adjacent_node_info_option: Option<&NodeInfo<N>> =
        self.node_to_node_info_map.get(&adjacent_node);
      let mut adjacent_node_info: NodeInfo<N> = match adjacent_node_info_option
      {
        None => {
          let adjacent_node_info = NodeInfo::new(adjacent_node);
          self.node_to_node_info_map.insert(adjacent_node, adjacent_node_info);
          self.open_node_info_sorted_list.push(adjacent_node_info);
          adjacent_node_info
        },
        Some(adjacent_node_info) => {
          if adjacent_node_info.cost_from_start <= new_cost_from_start {
            continue;
          }
          *adjacent_node_info
        },
      };
      self
        .node_to_parent_node_info_map
        .insert(adjacent_node_info.node, node_info);
      adjacent_node_info.cost_from_start = new_cost_from_start;
      let total_cost: f64 = new_cost_from_start
        + self.cartographer.borrow().estimate_cost_to_goal(&adjacent_node);
      adjacent_node_info.total_cost = total_cost;
      if total_cost < self.best_total_cost {
        self.best_node_info_option = Some(adjacent_node_info);
        self.best_total_cost = total_cost;
      }
      self.open_node_info_sorted_list.sort();
    }
    true
  }

  pub fn reset(
    &mut self,
    start_node: N,
  ) {
    self.goal_node_info_option = None;
    self.list_empty = false;
    self.open_node_info_sorted_list = Vec::new();
    self.node_to_node_info_map = HashMap::new();
    self.node_to_parent_node_info_map = HashMap::new();
    let start_node_info = NodeInfo::new(start_node);
    self.node_to_node_info_map.insert(start_node, start_node_info);
    self.open_node_info_sorted_list.push(start_node_info);
    self.best_total_cost = INFINITY;
  }
}
