// =============================================================================
//! - Traits for the A* algorithm
//!
//! # Metadata
//! - Copyright: &copy; 2022-2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Java created: 2002-04-21
//! - Java updated: 2003-04-29
//! - Rust created: 2022-10-21
//! - Rust updated: 2023-07-26
//!
//! # History
//! - Adapted from the interfaces in the Java-based [`CroftSoft Core Library`]
//!   - com.croftsoft.core.ai.astar.Cartographer
//!   - com.croftsoft.core.ai.astar.SpaceTester
//!
//! [`CroftSoft Core Library`]: https://www.CroftSoft.com/library/code/
//! [`CroftSoft Inc`]: https://www.CroftSoft.com/
//! [`David Wallace Croft`]: https://www.CroftSoft.com/people/david/
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
