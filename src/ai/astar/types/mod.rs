// =============================================================================
//! - Types for the A* algorithm
//!
//! # Metadata
//! - Copyright: &copy; 2022-2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust created: 2022-11-09
//! - Rust updated: 2023-07-26
//!
//! [`CroftSoft Inc`]: https://www.CroftSoft.com/
//! [`David Wallace Croft`]: https://www.CroftSoft.com/people/david/
// =============================================================================

pub type IsSpaceAvailableFunction<N> = fn(&N) -> bool;

pub type MakeNodeFunction<N> = fn(f64, f64) -> N;
