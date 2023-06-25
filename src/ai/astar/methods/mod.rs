// =============================================================================
//! - Methods for the A* algorithm
//!
//! # Metadata
//! - Copyright: &copy; 2022-2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Java created: 2002-04-21
//! - Java updated: 2003-05-09
//! - Rust created: 2022-10-28
//! - Rust updated: 2023-06-24
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
use core::f64::INFINITY;
use core::hash::Hash;
use std::collections::HashMap;
use std::collections::VecDeque;

impl<C: Cartographer<N>, N: Copy + Eq + Hash> AStar<C, N> {
  pub fn get_first_step(&self) -> Option<N> {
    let mut node_option: Option<N> = self.goal_node_option;
    if node_option.is_none() {
      node_option = self.best_node_option;
    }
    node_option?;
    let mut child_node_option: Option<N> = None;
    let mut node: N = node_option.unwrap();
    let mut parent_node_option = self.node_to_parent_node_map.get(&node);
    while parent_node_option.is_some() {
      child_node_option = node_option;
      node_option = parent_node_option.copied();
      node = node_option.unwrap();
      parent_node_option = self.node_to_parent_node_map.get(&node);
    }
    child_node_option
  }

  pub fn get_path(&self) -> VecDeque<N> {
    let mut path_list = VecDeque::new();
    let mut node_option: Option<N> = self.goal_node_option;
    if node_option.is_none() {
      node_option = self.best_node_option;
    }
    if node_option.is_none() {
      return path_list;
    }
    let mut node: N = node_option.unwrap();
    let mut parent_node_option = self.node_to_parent_node_map.get(&node);
    while parent_node_option.is_some() {
      path_list.push_front(node);
      node_option = parent_node_option.copied();
      node = node_option.unwrap();
      parent_node_option = self.node_to_parent_node_map.get(&node);
    }
    path_list
  }

  pub fn is_goal_found(&self) -> bool {
    self.goal_node_option.is_some()
  }

  pub fn loop_once(&mut self) -> bool {
    let Some(node) = self.open_node_sorted_list.pop_front() else {
      self.list_empty = true;
      return false;
    };
    let node_info: NodeInfo = *self.node_to_node_info_map.get(&node).unwrap();
    if self.cartographer.borrow().is_goal_node(&node) {
      if let Some(goal_node) = self.goal_node_option {
        let goal_node_info =
          self.node_to_node_info_map.get(&goal_node).unwrap();
        if goal_node_info.cost_from_start <= node_info.cost_from_start {
          return false;
        }
      }
      self.goal_node_option = Some(node);
      return false;
    }
    let adjacent_nodes: Vec<N> =
      self.cartographer.borrow().get_adjacent_nodes(&node);
    for adjacent_node in adjacent_nodes {
      let new_cost_from_start: f64 = node_info.cost_from_start
        + self
          .cartographer
          .borrow()
          .get_cost_to_adjacent_node(&node, &adjacent_node);
      let adjacent_node_info_option: Option<&NodeInfo> =
        self.node_to_node_info_map.get(&adjacent_node);
      if let Some(adjacent_node_info) = adjacent_node_info_option {
        if adjacent_node_info.cost_from_start <= new_cost_from_start {
          continue;
        }
        // TODO: Do something better here, maybe a sorted set
        let position_option = self
          .open_node_sorted_list
          .iter()
          .position(|&node| node == adjacent_node);
        if let Some(position) = position_option {
          self.open_node_sorted_list.remove(position);
        }
      }
      let total_cost: f64 = new_cost_from_start
        + self.cartographer.borrow().estimate_cost_to_goal(&adjacent_node);
      let adjacent_node_info = NodeInfo {
        cost_from_start: new_cost_from_start,
        total_cost,
      };
      self.node_to_node_info_map.insert(adjacent_node, adjacent_node_info);
      self.open_node_sorted_list.push_back(adjacent_node);
      self.open_node_sorted_list.make_contiguous().sort_by(|a, b| {
        let node_info_a = self.node_to_node_info_map.get(a).unwrap();
        let node_info_b = self.node_to_node_info_map.get(b).unwrap();
        node_info_a.cmp(node_info_b)
      });
      self.node_to_parent_node_map.insert(adjacent_node, node);
      if total_cost < self.best_total_cost {
        self.best_node_option = Some(adjacent_node);
        self.best_total_cost = total_cost;
      }
    }
    true
  }

  pub fn reset(
    &mut self,
    start_node: N,
  ) {
    self.goal_node_option = None;
    self.list_empty = false;
    self.open_node_sorted_list = VecDeque::new();
    self.node_to_node_info_map = HashMap::new();
    self.node_to_parent_node_map = HashMap::new();
    let start_node_info = NodeInfo::default();
    self.node_to_node_info_map.insert(start_node, start_node_info);
    self.open_node_sorted_list.push_front(start_node);
    self.best_total_cost = INFINITY;
  }
}
