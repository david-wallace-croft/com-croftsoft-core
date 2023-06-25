// =============================================================================
//! - Operation implementations for the A* algorithm
//!
//! # Metadata
//! - Copyright: &copy; 2022-2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Java created: 2002-04-21
//! - Java updated: 2003-05-09
//! - Rust created: 2022-10-22
//! - Rust updated: 2023-06-25
//!
//! # History
//! - Adapted from the classes in the Java-based [`CroftSoft Core Library`]
//!   - com.croftsoft.core.ai.astar.NodeInfo
//!
//! [`CroftSoft Core Library`]: https://www.croftsoft.com/library/code/
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::structures::NodeInfo;
use std::cmp::Ordering;

#[cfg(test)]
mod test;

impl PartialEq for NodeInfo {
  fn eq(
    &self,
    other: &Self,
  ) -> bool {
    self.total_cost.eq(&other.total_cost)
  }
}

impl PartialOrd for NodeInfo {
  fn lt(
    &self,
    other: &Self,
  ) -> bool {
    self.total_cost.lt(&other.total_cost)
  }

  fn le(
    &self,
    other: &Self,
  ) -> bool {
    self.total_cost.le(&other.total_cost)
  }

  fn gt(
    &self,
    other: &Self,
  ) -> bool {
    self.total_cost.gt(&other.total_cost)
  }

  fn ge(
    &self,
    other: &Self,
  ) -> bool {
    self.total_cost.ge(&other.total_cost)
  }

  fn partial_cmp(
    &self,
    other: &Self,
  ) -> Option<Ordering> {
    self.total_cost.partial_cmp(&other.total_cost)
  }
}
