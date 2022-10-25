// =============================================================================
//! - Trait implementations for the A* algorithm
//!
//! # Metadata
//! - Copyright: &copy; 2002 - 2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2022-10-25
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
use crate::math::geom::{structures::Point2DD, traits::PointXY};

impl<'a, 'b> Cartographer<Point2DD> for GridCartographer<'a, 'b, Point2DD> {
  fn estimate_cost_to_goal(
    &self,
    node: &Point2DD,
  ) -> f64 {
    node.distance_xy(self.goal_point_xy)
  }

  fn get_adjacent_nodes(
    &self,
    node: &Point2DD,
  ) -> Vec<Point2DD> {
    let mut adjacent_list = Vec::new();
    let distance_to_goal = node.distance_xy(self.goal_point_xy);
    if distance_to_goal <= self.step_size {
      let x = self.goal_point_xy.get_x();
      let y = self.goal_point_xy.get_y();
      adjacent_list.push(Point2DD {
        x,
        y,
      });
      return adjacent_list;
    }
    let x = node.get_x();
    let y = node.get_y();
    for ix in -1..2 {
      for iy in -1..2 {
        if ix == 0 && iy == 0 {
          continue;
        }
        let step = Point2DD {
          x: ((x / self.step_size).trunc() + ix as f64) * self.step_size,
          y: ((y / self.step_size).trunc() + iy as f64) * self.step_size,
        };
        if self.space_tester.is_space_available(&step) {
          adjacent_list.push(step);
        }
      }
    }
    adjacent_list
  }

  fn get_cost_to_adjacent_node(
    &self,
    from_node: &Point2DD,
    to_node: &Point2DD,
  ) -> f64 {
    from_node.distance_xy(to_node)
  }

  fn is_goal_node(
    &self,
    node: &Point2DD,
  ) -> bool {
    self.goal_point_xy.distance_xy(node) == 0.0
  }
}
