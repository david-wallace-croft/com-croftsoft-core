// =============================================================================
//! - PointXY for the geometry module
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Java created: 2003-03-20
//! - Java updated: 2003-04-13
//! - Rust created: 2023-03-18
//! - Rust updated: 2023-07-26
//!
//! # History
//! - Adapted from the interface in the Java-based [`CroftSoft Core Library`]
//!   - com.croftsoft.core.math.geom.PointXY
//!
//! [`CroftSoft Core Library`]: https://www.CroftSoft.com/library/code/
//! [`CroftSoft Inc`]: https://www.CroftSoft.com/
//! [`David Wallace Croft`]: https://www.CroftSoft.com/people/david/
// =============================================================================

use core::hash::Hash;

pub trait PointXY: Clone + Copy + Eq + Hash {
  /// The angle, in radians, from this point to the other point
  fn angle_to<P: PointXY>(
    &self,
    other_pointxy: &P,
  ) -> f64;

  fn distance(
    &self,
    other_x: f64,
    other_y: f64,
  ) -> f64;

  fn distance_xy<P: PointXY>(
    &self,
    other_pointxy: &P,
  ) -> f64;

  fn get_x(&self) -> f64;

  fn get_y(&self) -> f64;
}
