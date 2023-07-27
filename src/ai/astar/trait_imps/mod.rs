// =============================================================================
//! - Trait implementations for the A* algorithm
//!
//! # Metadata
//! - Copyright: &copy; 2022-2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Java created: 2002-04-21
//! - Java updated: 2003-05-10
//! - Rust created: 2022-10-24
//! - Rust updated: 2023-07-26
//!
//! # History
//! - Adapted from the classes in the Java-based [`CroftSoft Core Library`]
//!   - com.croftsoft.core.ai.astar.GridCartographer
//!
//! [`CroftSoft Core Library`]: https://www.CroftSoft.com/library/code/
//! [`CroftSoft Inc`]: https://www.CroftSoft.com/
//! [`David Wallace Croft`]: https://www.CroftSoft.com/people/david/
// =============================================================================

#[cfg(test)]
mod test;

use super::structures::AStar;
use super::structures::{GradientCartographer, GridCartographer, NodeInfo};
use super::traits::Cartographer;
use crate::math::geom::point_xy::PointXY;
use core::cmp::Ordering;
use core::f64::consts::TAU;
use core::f64::INFINITY;
use core::hash::Hash;
use std::collections::HashMap;
use std::collections::VecDeque;

impl<N: PointXY> Cartographer<N> for GradientCartographer<N> {
  fn estimate_cost_to_goal(
    &self,
    node: &N,
  ) -> f64 {
    node.distance_xy(&self.goal_node)
  }

  fn get_adjacent_nodes(
    &self,
    node: &N,
  ) -> Vec<N> {
    let mut adjacent_list = Vec::new();
    let distance_to_goal: f64 = node.distance_xy(&self.goal_node);
    let distance_from_start: f64 = node.distance_xy(&self.start_node);
    let step_size =
      (distance_from_start / self.init_step_size).trunc() * self.init_step_size;
    let step_size = step_size.max(self.init_step_size);
    if distance_to_goal <= step_size {
      let x: f64 = self.goal_node.get_x();
      let y: f64 = self.goal_node.get_y();
      let goal_node_copy: N = (self.make_node_fn)(x, y);
      adjacent_list.push(goal_node_copy);
      return adjacent_list;
    }
    let x: f64 = node.get_x();
    let y: f64 = node.get_y();
    let heading_to_goal =
      (self.goal_node.get_y() - y).atan2(self.goal_node.get_x() - x);
    let directions_f64 = self.directions as f64;
    for i in 0..self.directions {
      let heading = heading_to_goal + (i as f64) * TAU / directions_f64;
      let step: N = (self.make_node_fn)(
        x + step_size * heading.cos(),
        y + step_size * heading.sin(),
      );
      if (self.is_space_available_fn)(&step) {
        adjacent_list.push(step);
      }
    }
    adjacent_list
  }

  fn get_cost_to_adjacent_node(
    &self,
    from_node: &N,
    to_node: &N,
  ) -> f64 {
    from_node.distance_xy(to_node)
  }

  fn is_goal_node(
    &self,
    node: &N,
  ) -> bool {
    self.goal_node.distance_xy(node) == 0.0
  }
}

impl<N: PointXY> Cartographer<N> for GridCartographer<N> {
  fn estimate_cost_to_goal(
    &self,
    node: &N,
  ) -> f64 {
    node.distance_xy(&self.goal_node)
  }

  fn get_adjacent_nodes(
    &self,
    node: &N,
  ) -> Vec<N> {
    let mut adjacent_list = Vec::new();
    let distance_to_goal: f64 = node.distance_xy(&self.goal_node);
    if distance_to_goal <= self.step_size {
      let x: f64 = self.goal_node.get_x();
      let y: f64 = self.goal_node.get_y();
      let goal_node_copy: N = (self.make_node_fn)(x, y);
      adjacent_list.push(goal_node_copy);
      return adjacent_list;
    }
    let x: f64 = node.get_x();
    let y: f64 = node.get_y();
    for ix in -1..2 {
      for iy in -1..2 {
        if ix == 0 && iy == 0 {
          continue;
        }
        let step: N = (self.make_node_fn)(
          ((x / self.step_size).trunc() + ix as f64) * self.step_size,
          ((y / self.step_size).trunc() + iy as f64) * self.step_size,
        );
        if (self.is_space_available_fn)(&step) {
          adjacent_list.push(step);
        }
      }
    }
    adjacent_list
  }

  fn get_cost_to_adjacent_node(
    &self,
    from_node: &N,
    to_node: &N,
  ) -> f64 {
    from_node.distance_xy(to_node)
  }

  fn is_goal_node(
    &self,
    node: &N,
  ) -> bool {
    self.goal_node.distance_xy(node) == 0.0
  }
}

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

impl Eq for NodeInfo {}

impl Ord for NodeInfo {
  fn cmp(
    &self,
    other: &Self,
  ) -> std::cmp::Ordering {
    if self.total_cost < other.total_cost {
      return Ordering::Less;
    } else if self.total_cost > other.total_cost {
      return Ordering::Greater;
    }
    Ordering::Equal
  }
}
