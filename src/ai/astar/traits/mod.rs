// =============================================================================
//! - Traits for the A* algorithm
//!
//! # Metadata
//! - Copyright: &copy; 2002 - 2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2022-11-10
//! - Rust since: 2022-10-21
//! - Java version: 2003-04-29
//! - Java since: 2002-04-21
//!
//! # History
//! - Adapted from the interfaces in the Java-based [`CroftSoft Core Library`]
//!   - com.croftsoft.core.ai.astar.Cartographer
//!   - com.croftsoft.core.ai.astar.SpaceTester
//!
//! [`CroftSoft Core Library`]: https://www.croftsoft.com/library/code/
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

/// An A* algorithm map maker
pub trait Cartographer<N> {
  fn estimate_cost_to_goal(
    &self,
    node: &N,
  ) -> f64;

  fn get_adjacent_nodes(
    &self,
    node: &N,
  ) -> Vec<N>;

  fn get_cost_to_adjacent_node(
    &self,
    from_node: &N,
    to_node: &N,
  ) -> f64;

  fn is_goal_node(
    &self,
    node: &N,
  ) -> bool;
}
