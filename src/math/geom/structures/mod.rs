// =============================================================================
//! - Structures for the geometry module
//!
//! # Metadata
//! - Copyright: &copy; 2003 - 2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2022-11-10
//! - Rust since: 2022-10-23
//! - Java version: 2003-04-13
//! - Java since: 2003-03-20
//!
//! # History
//! - Adapted from the classes in the Java-based [`CroftSoft Core Library`]
//!   - com.croftsoft.core.math.geom.Point2DD
//!
//! [`CroftSoft Core Library`]: https://www.croftsoft.com/library/code/
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

#[derive(Clone, Copy, Debug, Default)]
pub struct Point2DD {
  pub x: f64,
  pub y: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Rectangle {
  pub x_max: f64,
  pub x_min: f64,
  pub y_max: f64,
  pub y_min: f64,
}
