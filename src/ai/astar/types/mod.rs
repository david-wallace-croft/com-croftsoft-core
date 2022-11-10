// =============================================================================
//! - Types for the A* algorithm
//!
//! # Metadata
//! - Copyright: &copy; 2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2022-11-10
//! - Rust since: 2022-11-09
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

pub type IsSpaceAvailableFunction<N> = fn(&N) -> bool;

pub type MakeNodeFunction<N> = fn(f64, f64) -> N;
