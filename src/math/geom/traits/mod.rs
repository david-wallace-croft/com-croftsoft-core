// =============================================================================
//! - Traits for the geometry module
//!
//! # Metadata
//! - Copyright: &copy; 2003 - 2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2022-10-23
//! - Rust since: 2022-10-23
//! - Java version: 2003-04-13
//! - Java since: 2003-03-20
//!
//! # History
//! - Adapted from the interfaces in the Java-based [`CroftSoft Core Library`]
//!   - com.croftsoft.core.math.geom.PointXY
//!
//! [`CroftSoft Core Library`]: https://www.croftsoft.com/library/code/
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

pub trait PointXY {
  /// The angle, in radians, from this point to the other point
  fn angle_to(
    &self,
    other_pointxy: &dyn PointXY,
  ) -> f64;

  fn distance(
    &self,
    other_x: f64,
    other_y: f64,
  ) -> f64;

  fn distance_xy(
    &self,
    other_pointxy: &dyn PointXY,
  ) -> f64;

  fn get_x(&self) -> f64;

  fn get_y(&self) -> f64;
}
