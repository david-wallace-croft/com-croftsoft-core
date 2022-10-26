// =============================================================================
//! - Trait implementations for the A* algorithm
//!
//! # Metadata
//! - Copyright: &copy; 2002 - 2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2022-10-26
//! - Rust since: 2022-10-24
//! - Java version: 2003-05-10
//! - Java since: 2002-04-21
//!
//! # History
//! - Adapted from the classes in the Java-based [`CroftSoft Core Library`]
//!   - com.croftsoft.core.ai.astar.GridCartographer
//!
//! [`CroftSoft Core Library`]: https://www.croftsoft.com/library/code/
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::{
  structures::GridCartographer,
  traits::{Cartographer, NodeFactory, SpaceTester},
};
use crate::math::geom::traits::PointXY;

impl<'f, 'n, 's, F: NodeFactory<N>, N: PointXY, S: SpaceTester<N>>
  Cartographer<N> for GridCartographer<'f, 'n, 's, F, N, S>
{
  fn estimate_cost_to_goal(
    &self,
    node: &N,
  ) -> f64 {
    node.distance_xy(self.goal_node)
  }

  fn get_adjacent_nodes(
    &self,
    node: &N,
  ) -> Vec<N> {
    let mut adjacent_list = Vec::new();
    let distance_to_goal: f64 = node.distance_xy(self.goal_node);
    if distance_to_goal <= self.step_size {
      let x: f64 = self.goal_node.get_x();
      let y: f64 = self.goal_node.get_y();
      adjacent_list.push(self.node_factory.make_node(x, y));
      return adjacent_list;
    }
    let x: f64 = node.get_x();
    let y: f64 = node.get_y();
    for ix in -1..2 {
      for iy in -1..2 {
        if ix == 0 && iy == 0 {
          continue;
        }
        let step: N = self.node_factory.make_node(
          ((x / self.step_size).trunc() + ix as f64) * self.step_size,
          ((y / self.step_size).trunc() + iy as f64) * self.step_size,
        );
        if self.space_tester.is_space_available(&step) {
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
