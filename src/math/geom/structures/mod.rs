// =============================================================================
//! - Structures for the geometry module
//!
//! # Metadata
//! - Copyright: &copy; 2022-2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Java created: 2003-03-20
//! - Java updated: 2003-04-13
//! - Rust created: 2022-10-23
//! - Rust updated: 2023-03-12
//!
//! # History
//! - Adapted from the classes in the Java-based [`CroftSoft Core Library`]
//!   - com.croftsoft.core.math.geom.Circle
//!   - com.croftsoft.core.math.geom.Point2DD
//!
//! [`CroftSoft Core Library`]: https://www.croftsoft.com/library/code/
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

#[derive(Clone, Copy, Debug, Default)]
pub struct Circle {
  pub center_x: f64,
  pub center_y: f64,
  pub radius: f64,
}

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
