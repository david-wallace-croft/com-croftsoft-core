// =============================================================================
//! - Traits that define roles
//!
//! # Metadata
//! - Copyright: &copy; 2022-2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Java updated: 2003-04-09
//! - Rust created: 2022-10-09
//! - Rust updated: 2023-07-26
//!
//! # History
//! - Adapted from the Java interfaces in package com.croftsoft.core.role
//!   - In the Java-based [`CroftSoft Core Library`]
//!
//! [`CroftSoft Core Library`]: https://www.CroftSoft.com/library/code/
//! [`CroftSoft Inc`]: https://www.CroftSoft.com/
//! [`David Wallace Croft`]: https://www.CroftSoft.com/people/david/
// =============================================================================

#[cfg(test)]
mod test;

/// Determines whether an item should be filtered
pub trait Filter<I> {
  /// Filtrate is what passes through a filter
  fn is_filtrate(
    &self,
    item: I,
  ) -> bool;
}

/// A generic trait for request processors
pub trait Server<R, S> {
  /// Processes a request and returns a response
  fn serve(
    &self,
    request: R,
  ) -> S;
}
