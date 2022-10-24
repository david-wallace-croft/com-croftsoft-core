// =============================================================================
//! - Trait implementations for the A* algorithm
//!
//! # Metadata
//! - Copyright: &copy; 2002 - 2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2022-10-24
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

use super::{structures::GridCartographer, traits::Cartographer};
use crate::math::geom::traits::PointXY;

impl<'a, 'b, 'c, N: PointXY> Cartographer<N>
  for GridCartographer<'a, 'b, 'c, N>
{
  fn estimate_cost_to_goal(
    &self,
    node: &N,
  ) -> f64 {
    node.distance_xy(self.goal_point_xy)
  }

  fn get_adjacent_nodes(
    &self,
    _node: &N,
  ) -> &[N] {
    todo!()
    // self.adjacent_list.clear();
    // let distance_to_goal = node.distance_xy(self.goal_point_xy);
    // if distance_to_goal <= self.step_size {
    //   self.adjacent_list.push(self.goal_point_xy.clone());
    // }
    // // TODO: return iterator or slice instead
    // self.adjacent_list.as_slice()
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
    self.goal_point_xy.distance_xy(node) == 0.0
  }
}
